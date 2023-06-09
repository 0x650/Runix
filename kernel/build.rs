use std::env;

fn main() {
    let cwd_path = env::current_dir().expect("cannot get current directory");
    let cwd = cwd_path.display();
    println!("cargo:rerun-if-changed={cwd}/build.rs");
    println!("cargo:rerun-if-changed={cwd}/Cargo.toml");
    println!("cargo:rerun-if-changed={cwd}/Cargo.lock");
    println!("cargo:rerun-if-changed={cwd}/src");
    println!("cargo:rerun-if-changed={cwd}/linker.ld");
    // Tell cargo to pass the linker script to the linker..
    println!("cargo:rustc-link-arg=-Tlinker.ld");
    // ..and to re-run if it changes.
    println!("cargo:rerun-if-changed=linker.ld");
}
