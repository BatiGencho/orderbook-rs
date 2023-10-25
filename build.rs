use glob::glob;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //TODO add download external protos files instead of doing it manually
    let protos = find_protos("./protos");

    let mut prost_config = prost_build::Config::default();
    prost_config.protoc_arg("--experimental_allow_proto3_optional");

    tonic_build::configure()
        .build_server(true)
        .build_transport(true)
        .build_client(true)
        .file_descriptor_set_path("src/grpc/api.bin")
        .compile_with_config(
            prost_config,
            &protos,
            &["protos"], // specify the root location to search proto dependencies
        )?;

    Ok(())
}

fn find_protos(dir_path: &str) -> Vec<PathBuf> {
    glob(&format!("{dir_path}/**/*.proto"))
        .unwrap()
        .flatten()
        .collect()
}
