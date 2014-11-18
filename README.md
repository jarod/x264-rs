### use x264-rs
```toml
[dependencies.x264-rs]
git = "https://github.com/jarod/x264-rs.git"
```

### try build for linux
```bash
# install yasm
cargo build
```

### try build for android
```bash
# install android NDK
export NDK_HOME="path to android NDK"
export PATH=$NDK_HOME/toolchains/arm-linux-androideabi-$TOOLCHAIN_VERSION/prebuilt/linux-x86_64/bin:$PATH
export NDK_SYSROOT=$NDK_HOME/platforms/android-$ANDROID_API_VERSION/arch-arm
cargo build --target=arm-linux-androideabi
```
