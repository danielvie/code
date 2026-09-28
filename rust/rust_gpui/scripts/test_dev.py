"""Watcher checks without opening windows or terminating real processes."""
import contextlib
import io
from pathlib import Path
import tempfile
import unittest
from unittest.mock import Mock, patch

import dev


class DevTests(unittest.TestCase):
    def test_watch_scope_includes_added_deleted_sources_but_not_autosave(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "src").mkdir()
            (root / "Cargo.toml").write_text("manifest")
            before = dev.fingerprint(root)
            (root / ".local").mkdir()
            (root / ".local" / "diagram.json").write_text("saved")
            (root / "target").mkdir()
            (root / "target" / "binary.exe").write_text("compiled")
            self.assertEqual(dev.fingerprint(root), before)
            source = root / "src" / "new.rs"
            source.write_text("fn main() {}")
            self.assertNotEqual(dev.fingerprint(root), before)
            source.unlink()
            self.assertEqual(dev.fingerprint(root), before)

    def test_failed_build_keeps_app_and_next_success_replaces_it(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            binary = root / "app.exe"
            binary.write_bytes(b"fake executable")
            generation = [0]
            first, second = Mock(), Mock()
            first.poll.return_value = second.poll.return_value = None

            def tick(_):
                if generation[0] < 2:
                    self.assertEqual(launch.call_count, 1)
                    stop.assert_not_called()
                else:
                    self.assertEqual(launch.call_count, 2)
                    stop.assert_called_once_with(first)
                    raise KeyboardInterrupt
                generation[0] += 1

            with patch.object(dev, "ROOT", root), \
                 patch.object(dev, "fingerprint", side_effect=lambda: (generation[0],)), \
                 patch.object(dev, "stable_fingerprint", side_effect=lambda: (generation[0],)), \
                 patch.object(dev, "build", side_effect=[binary, None, binary]), \
                 patch.object(dev, "spawn", side_effect=[first, second]) as launch, \
                 patch.object(dev, "stop") as stop, \
                 patch.object(dev.time, "sleep", side_effect=tick), \
                 contextlib.redirect_stdout(io.StringIO()):
                dev.run()
                self.assertEqual([call.args[0] for call in stop.call_args_list], [first, second])
            self.assertEqual(list((root / "target" / "dev-runs").iterdir()), [])

    def test_edit_during_build_does_not_launch_stale_binary(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            binary = root / "app.exe"
            binary.write_bytes(b"fake executable")
            generation = [0]
            app = Mock()
            app.poll.return_value = None

            def build():
                generation[0] = 1
                return binary

            with patch.object(dev, "ROOT", root), \
                 patch.object(dev, "fingerprint", side_effect=lambda: (generation[0],)), \
                 patch.object(dev, "stable_fingerprint", side_effect=lambda: (generation[0],)), \
                 patch.object(dev, "build", side_effect=build) as compile_app, \
                 patch.object(dev, "spawn", return_value=app) as launch, \
                 patch.object(dev, "stop") as stop, \
                 patch.object(dev.time, "sleep", side_effect=KeyboardInterrupt), \
                 contextlib.redirect_stdout(io.StringIO()):
                dev.run()
                self.assertEqual(compile_app.call_count, 2)
                launch.assert_called_once()
                stop.assert_any_call(app)

    def test_build_finds_executable_from_cargo_messages(self):
        process = Mock()
        process.stdout = io.StringIO('{"reason":"compiler-artifact","target":{"name":"gpui_diagram_poc"},"executable":"build/app.exe"}\n')
        process.wait.return_value = 0
        with patch.object(dev, "spawn", return_value=process), patch.object(dev, "stop"), contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(dev.build(), Path("build/app.exe"))
        self.assertTrue(process.stdout.closed)


if __name__ == "__main__":
    unittest.main()
