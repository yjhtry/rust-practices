use std::process::Command;

fn main() {
    prost_build::Config::new()
        .out_dir("src/pb")
        .type_attribute(".", "#[derive(PartialOrd)]")
        // .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile_protos(&["proto/abi.proto"], &["."])
        .unwrap();

    Command::new("cargo")
        .args(["fmt", "--", "src/*.rs"])
        .status()
        .expect("cargo fmt failed");
}
