//! Versioned, atomic autosave. A separate lock file prevents competing writers.
use crate::diagram_model::{Diagram, Pos};
use crate::diagram_validate::validate_diagram;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

const VERSION: u32 = 1;

#[derive(Deserialize)]
pub struct Snapshot {
    pub version: u32,
    pub diagram: Diagram,
    pub pan: Pos,
    pub zoom: f32,
}

#[derive(Serialize)]
struct SnapshotRef<'a> {
    version: u32,
    diagram: &'a Diagram,
    pan: Pos,
    zoom: f32,
}

pub struct Store {
    path: PathBuf,
    // Keep the lock for the lifetime of the application, including atomic replacements.
    _lock: File,
    saved: Vec<u8>,
}

impl Store {
    pub fn default_path() -> PathBuf {
        std::env::var_os("GPUI_DIAGRAM_STATE")
            .map(PathBuf::from)
            .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(".local/diagram.json"))
    }

    pub fn open(path: PathBuf) -> io::Result<(Self, Snapshot)> {
        let path = std::path::absolute(path)?;
        fs::create_dir_all(path.parent().unwrap())?;
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path.with_extension("lock"))?;
        lock.try_lock().map_err(|e| {
            io::Error::other(format!(
                "Cannot lock {}: {e}. Close the other diagram app first.",
                path.display()
            ))
        })?;
        let snapshot = match fs::read(&path) {
            Ok(bytes) => {
                let snapshot: Snapshot = serde_json::from_slice(&bytes).map_err(|e| {
                    io::Error::other(format!(
                        "Invalid save {}: {e}. The file has not been changed.",
                        path.display()
                    ))
                })?;
                validate(&snapshot).map_err(|e| {
                    io::Error::other(format!(
                        "Invalid save {}: {e}. The file has not been changed.",
                        path.display()
                    ))
                })?;
                snapshot
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => Snapshot {
                version: VERSION,
                diagram: Diagram::demo(),
                pan: Pos::new(36., 24.),
                zoom: 1.,
            },
            Err(e) => return Err(e),
        };
        let mut store = Self {
            path,
            _lock: lock,
            saved: vec![],
        };
        store.save(&snapshot.diagram, snapshot.pan, snapshot.zoom)?;
        Ok((store, snapshot))
    }

    pub fn save(&mut self, diagram: &Diagram, pan: Pos, zoom: f32) -> io::Result<()> {
        let bytes = serde_json::to_vec_pretty(&SnapshotRef {
            version: VERSION,
            diagram,
            pan,
            zoom,
        })?;
        if bytes == self.saved {
            return Ok(());
        }
        let mut temp = tempfile::NamedTempFile::new_in(self.path.parent().unwrap())?;
        temp.write_all(&bytes)?;
        temp.as_file().sync_all()?;
        // tempfile replaces the destination atomically on Windows as well as Unix.
        temp.persist(&self.path).map_err(|e| e.error)?;
        self.saved = bytes;
        Ok(())
    }
}

fn validate(s: &Snapshot) -> Result<(), &'static str> {
    if s.version != VERSION {
        return Err("unsupported schema version");
    }
    if !s.pan.x.is_finite()
        || !s.pan.y.is_finite()
        || !s.zoom.is_finite()
        || !(0.4..=1.8).contains(&s.zoom)
    {
        return Err("invalid viewport");
    }
    validate_diagram(&s.diagram)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn round_trip_and_replacement_preserve_diagram_and_view() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("diagram.json");
        let (mut store, mut s) = Store::open(path.clone()).unwrap();
        s.diagram.block_mut(0).name = "My power supply".into();
        s.diagram.block_mut(0).pos = Pos::new(120., 240.);
        s.diagram.delete_block(5);
        s.diagram.add(Pos::new(900., 600.));
        let pan = Pos::new(-150., 27.);
        store.save(&s.diagram, pan, 0.75).unwrap();
        let bytes = fs::read(&path).unwrap();
        store.save(&s.diagram, pan, 0.75).unwrap();
        assert_eq!(fs::read(&path).unwrap(), bytes);
        drop(store);
        let (_store, restored) = Store::open(path).unwrap();
        assert_eq!(
            serde_json::to_value(&restored.diagram).unwrap(),
            serde_json::to_value(&s.diagram).unwrap()
        );
        assert_eq!(restored.pan, pan);
        assert_eq!(restored.zoom, 0.75);
    }
    #[test]
    fn refuses_second_writer_until_first_exits() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("diagram.json");
        let (store, _) = Store::open(path.clone()).unwrap();
        assert!(Store::open(path.clone()).is_err());
        drop(store);
        assert!(Store::open(path).is_ok());
    }
    #[test]
    fn invalid_and_future_files_are_not_overwritten() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("diagram.json");
        for bytes in [
            b"not json".to_vec(),
            serde_json::to_vec(&SnapshotRef {
                version: 99,
                diagram: &Diagram::demo(),
                pan: Pos::default(),
                zoom: 1.,
            })
            .unwrap(),
        ] {
            fs::write(&path, &bytes).unwrap();
            assert!(Store::open(path.clone()).is_err());
            assert_eq!(fs::read(&path).unwrap(), bytes);
        }
    }
    #[test]
    fn rejects_dangling_ports_off_grid_positions_and_bad_ids() {
        let mut s = Snapshot {
            version: VERSION,
            diagram: Diagram::demo(),
            pan: Pos::default(),
            zoom: 1.,
        };
        assert!(validate(&s).is_ok());
        s.diagram.wires[0].to.index = 100;
        assert!(validate(&s).is_err());
        s.diagram = Diagram::demo();
        s.diagram.blocks[0].pos.x = 1.;
        assert!(validate(&s).is_err());
        s.diagram = Diagram::demo();
        s.diagram.next_id = 0;
        assert!(validate(&s).is_err());
    }
}
