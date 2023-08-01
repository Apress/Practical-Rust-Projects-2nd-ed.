# Cross compiling

```
rustup target add armv7-unknown-linux-gnueabihf
sudo apt-get install gcc-10-multilib-arm-linux-gnueabihf
```

Add the folloing to the `~/.cargo/config` file (create one if it does not exist):

```
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc-10"
```

```
cargo build --target armv7-unknown-linux-gnueabihf
```

Then copy `target/armv7-unknown-linux-gnueabihf/debug/blink-cross-compile` to the Raspberry Pi


