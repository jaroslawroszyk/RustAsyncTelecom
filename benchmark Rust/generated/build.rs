use protobuf_codegen::Codegen;

fn main() {
    Codegen::new()
        .pure()
        .out_dir("src/")
        .input("../idl/ap.proto")
        .include("../idl")
        .run()
        .expect("Codegen failed.")
}
