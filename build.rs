use const_format::formatcp;

const WASI_SDK: &str = "/opt/wasi-sdk";
const WASI_VERSION: &str = "14.0.4";

const CC: &str = formatcp!("{WASI_SDK}/bin/clang");
const CXX: &str = formatcp!("{WASI_SDK}/bin/clang++");
const LD: &str = formatcp!("{WASI_SDK}/bin/wasm-ld");


fn main() {
    std::env::set_var("AR", "llvm-ar");
    //    std::env::set_var("CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER", LD);

    println!("cargo:rustc-link-search={WASI_SDK}/share/wasi-sysroot/lib/wasm32-wasi");
    println!("cargo:rustc-link-search={WASI_SDK}/lib/clang/{WASI_VERSION}/lib/wasi");
    
    println!("cargo:rustc-link-arg=--no-entry");
    println!("cargo:rustc-link-arg=--import-undefined");
    println!("cargo:rustc-link-arg=--export-dynamic");
    // println!("cargo:rustc-link-arg=--import-memory");
    println!("cargo:rustc-link-arg=-lc");
    println!("cargo:rustc-link-arg=-lc++");
    println!("cargo:rustc-link-arg=-lc++abi");
    println!("cargo:rustc-link-arg=-lclang_rt.builtins-wasm32");

    cc::Build::new()
        .compiler(CC)
        .file("kek.c")
        .compile("kek");

    cc::Build::new()
        .compiler(CXX)
        .file("lol.cpp")
        .compile("lol");
}
