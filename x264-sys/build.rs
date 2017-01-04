extern crate bindgen;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

fn bindgen_x264_h(out_dir: &str) {
    let mut bindings = bindgen::Builder::new("src/x264_bindgen.h");
    bindings.clang_arg("-Ix264")
        .clang_arg(format!("-I{}",out_dir))
        .builtins();
    let generated_bindings = bindings.generate().expect("Failed to generate x264.h bindings");
    let mut file = File::create("src/ffi.rs").expect("Failed to open src/ffi.rs");
    file.write(generated_bindings.to_string().as_bytes()).unwrap();
}

fn main() {
    let src_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let src = Path::new(&src_dir);
    let dst_dir = env::var("OUT_DIR").unwrap();
    let dst = Path::new(&dst_dir);
    let target = env::var("TARGET").unwrap();

    let mut configure = Command::new("bash");
    configure.arg(src.join("x264/configure"));
    if target.contains("android") {
        let ndk_dir = env::var("ANDROID_NDK").unwrap();
        let ndk = Path::new(&ndk_dir);
        configure.arg(format!("--cross-prefix={}-", target))
                 .arg(format!("--sysroot={}", ndk.join("sysroot").display()))
                 .arg("--host=arm-linux");
    }
    run(configure.current_dir(&dst)
                 .arg(format!("--prefix={}", dst.display()))
                 .arg("--enable-pic")
                 .arg("--enable-static")
                 .arg("--disable-cli"));
    run(Command::new("make").arg(format!("-j{}", 3)).current_dir(&dst));
    println!("cargo:rustc-flags=-L {} -l x264:static", dst.display());

    bindgen_x264_h(&dst_dir);
}

fn run(cmd: &mut Command) {
    assert!(cmd.stdout(Stdio::inherit())
               .stderr(Stdio::inherit())
               .status()
               .unwrap()
               .success());

}
