use std::process::Command;

fn main() {
  tonic_build::configure()
    .compile_well_known_types(true)
    .out_dir("src/pb")
    .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    .compile(&["../../protos/musync.proto"], &["../../protos"])
    .unwrap();

  Command::new("cargo").args(["fmt"]).output().unwrap();

  println!("cargo:rerun-if-changed=../../protos/musync.proto")
}
