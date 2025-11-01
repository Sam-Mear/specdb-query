
fn main() {
    prost_build::compile_protos(
        &["src/queries/protobuf/cpu.proto"],
        &["src/queries/protobuf"],
    ).unwrap();
}
