### use x264-rs
```toml
[dependencies]
x264 = { git="https://github.com/jarod/x264-rs.git" }
```

### try build for linux
```bash
# install yasm
cargo build
```

### try build for android
```bash
# follow https://github.com/rust-lang/rust/wiki/Doc-building-for-android to setup rust for android and standalone ndk toolchain
export ANDROID_PATH=<standalone ndk toolchain path>
export PATH=$ANDROID_NDK/bin:$PATH
cargo build --target=arm-linux-androideabi
```
