use std::fs;

fn main() {
    // We let PROST generate into src/generated. We can include it from there.
    let _ = fs::create_dir_all("src/generated");

    prost_build::Config::new()
        .out_dir("src/generated")
        .compile_protos(&["../schemas/sensor.proto"], &["../schemas"])
        .unwrap();
}
