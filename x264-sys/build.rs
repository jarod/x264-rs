use std::io::Command;
use std::os;
use std::io::process::InheritFd;

fn main() {
    let src = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
    let dst = Path::new(os::getenv("OUT_DIR").unwrap());
    let target = os::getenv("TARGET").unwrap();

    let mut configure = Command::new("bash");
    configure.arg(src.join("x264/configure"));
    if target.contains("android") {
      let ndk = Path::new(os::getenv("ANDROID_NDK").unwrap());
    	configure
      	.arg(format!("--cross-prefix={}-", target))
      	.arg(format!("--sysroot={}", ndk.join("sysroot").display()))
      	.arg("--host=arm-linux");
    }
	run(configure
		.cwd(&dst)
		.arg(format!("--prefix={}", dst.display()))
		.arg("--enable-pic")
		.arg("--enable-static")
		.arg("--disable-cli"));
	run(Command::new("make").arg(format!("-j{}", os::num_cpus())).cwd(&dst));
	println!("cargo:rustc-flags=-L {} -l x264:static", dst.display());
}

fn run(cmd: &mut Command) {
    println!("running: {}", cmd);
    assert!(cmd.stdout(InheritFd(1))
               .stderr(InheritFd(2))
               .status()
               .unwrap()
               .success());

}