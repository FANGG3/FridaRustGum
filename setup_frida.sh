export C_INCLUDE_PATH=$C_INCLUDE_PATH:$(pwd)/c
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(pwd)/c
export BINDGEN_EXTRA_CLANG_ARGS='--sysroot /home/fang/.cargo/NDK/arm64/sysroot'
#export CARGO_BUILD_TARGET="aarch64-linux-android"
#  cargo build --target aarch64-linux-android