use std::env;
fn setup_android_workaround() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS not set");
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH not set");
    if (target_arch == "x86_64" || target_arch == "aarch64") && target_os == "android" {
        let android_ndk_home = env::var("ANDROID_NDK_HOME").expect("ANDROID_NDK_HOME not set");
        let build_os = match env::consts::OS {
            "linux" => "linux",
            "macos" => "darwin",
            "windows" => "windows",
            _ => panic!(
                "Unsupported OS. You must use either Linux, MacOS or Windows to build the crate."
            ),
        };
        const DEFAULT_CLANG_VERSION: &str = "17";
        let clang_version =
            env::var("NDK_CLANG_VERSION").unwrap_or_else(|_| DEFAULT_CLANG_VERSION.to_owned());
        let linux_x86_64_lib_dir = "toolchains/llvm/prebuilt/linux-x86_64/lib/clang/17/lib/linux/";
        println!("cargo:rustc-link-search=/home/fang/workspace/env/android-ndk-r26d/toolchains/llvm/prebuilt/linux-x86_64/lib/clang/17/lib/linux");
        println!("cargo:rustc-link-lib=static=clang_rt.builtins-aarch64-android");
    }
}

fn main() {
    println!("cargo:rust-link-lib=static=frida-gum");
    println!("cargo:rustc-link-search=./c");
    setup_android_workaround();
}
