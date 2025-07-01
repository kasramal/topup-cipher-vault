use tonic_build;

fn main() {
    tonic_build::configure()
        .compile(
            &["../proto-buf-lib/src/main/proto/vault.proto"],
            &["../proto-buf-lib/src/main/proto"]
        )
        .expect("Failed to compile protos");
}