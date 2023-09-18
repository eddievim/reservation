fn main() {
    tonic_build::configure()
    .out_dir("src/pb")
    .compile(&["protos/reservation.proto"], &["protos"])
    .unwrap();


    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rustc-env=ABI={}", std::env::var("CARGO_PKG_VERSION").unwrap());        
}
 