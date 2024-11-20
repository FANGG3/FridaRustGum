use std::env;
fn setup_android_workaround() {

    println!("C_INCLUDE_PATH {}",env::var("C_INCLUDE_PATH").unwrap());
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS not set");
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH not set");
    if (target_arch == "x86_64" || target_arch == "aarch64") && target_os == "android" {
        let android_ndk_home = String::from("/Users/fangg3/.cargo/NDK/")+&target_arch;
        let build_os = match env::consts::OS {
            "linux" => "linux",
            "macos" => "darwin",
            "windows" => "windows",
            _ => panic!(
                "Unsupported OS. You must use either Linux, MacOS or Windows to build the crate."
            ),
        };
        const DEFAULT_CLANG_VERSION: &str = "12.0.9";
        let clang_version =
            env::var("NDK_CLANG_VERSION").unwrap_or_else(|_| DEFAULT_CLANG_VERSION.to_owned());
        let linux_x86_64_lib_dir = format!("{android_ndk_home}/lib64/clang/{clang_version}/lib/linux");
        println!("cargo:rustc-link-search={linux_x86_64_lib_dir}");
        println!("cargo:rustc-link-lib=static=clang_rt.builtins-aarch64-android");
    }
}

fn main() {
    println!("cargo:include=./c");
    println!("cargo:rust-link-lib=static=frida-gum");
    println!("cargo:rustc-link-search=./c");
    setup_android_workaround();
}
