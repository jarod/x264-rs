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
# follow https://github.com/rust-lang/rust/wiki/Doc-building-for-android to setup standalone ndk toolchain
export ANDROID_PATH=<standalone ndk toolchain path>
export PATH=$ANDROID_NDK:$PATH
cargo build --target=arm-linux-androideabi
```
