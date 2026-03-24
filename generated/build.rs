use protobuf_codegen::Codegen;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    Codegen::new()
        .pure()
        .out_dir("src/")
        .input("../idl/communication.proto")
        .include("../idl")
        .run()?;

    println!("cargo:rerun-if-changed=../idl/communication.proto");

    Ok(())
}
