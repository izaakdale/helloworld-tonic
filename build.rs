use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let original_out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let out_dir = "./src";

    tonic_build::configure()
        .out_dir(out_dir)
        .file_descriptor_set_path(original_out_dir.join("helloworld.bin"))
        .compile(&["proto/helloworld.proto"], &["proto"])?;

    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}