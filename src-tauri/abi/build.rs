use std::process::Command;

fn main() {
  prost_build::Config::new()
    .out_dir("src/pb")
    .compile_protos(&["protos/musync.proto"], &["protos"])
    .unwrap();

  Command::new("cargo").args(&["fmt"]).output().unwrap();
}
