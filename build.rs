fn main() {
    #[cfg(feature = "build")]
    let protos = &["proto/nostr.proto"];

    #[cfg(feature = "build")]
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src")
        .compile_protos(protos, &["proto"])
        .expect("failed to compile arkd server protos");
}
