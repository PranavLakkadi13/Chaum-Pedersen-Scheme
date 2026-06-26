use std::error::Error;

fn main() {
    tonic_prost_build::configure()
        .build_server(true)
        .out_dir("src/")
        .compile_protos(&["proto/zkp_auth.proto"], &["proto/"])
        .unwrap();
}
