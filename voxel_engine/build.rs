fn main() {
    println!("!cargo:rerun-if-changed=shaders/shaders.hlsl");
    std::fs::copy(
        "shaders/shaders.hlsl",
        std::env::var("OUT_DIR").unwrap() + "/../../../shaders.hlsl",
    )
    .expect("Copy");
}