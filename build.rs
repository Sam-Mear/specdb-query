
fn main() {
    prost_build::compile_protos(
        &["src/queries/protobuf/query.proto"],
        &["src/queries/protobuf"],
    ).unwrap();
}
