# Raspberry Pi Zero W | Remote Device Control Server (Rust)

Server application for Raspberry Pi Zero W which allows to control remotely a led. The client side can be found at this [link](https://github.com/goto-eof/rpizero_rdc_ts).

Technologies/Tools: Rust, tide web server, rust_gpiozero.

### Build on MacOS, run on Raspberry Pi Zero W

Here are the instructions to build on MacOS in order to run then the executable on Raspberry Pi Zero W. The compiled file is under `target/arm-unknown-linux-musleabi/release`.

```
rustup target list | grep "armv-"

brew install arm-linux-gnueabihf-binutils

rustup target add arm-unknown-linux-musleabi

cargo build --target=arm-unknown-linux-musleabi --release
```

For [more configuration info](https://amritrathie.vercel.app/posts/2020/03/06/cross-compiling-rust-from-macos-to-raspberry-pi/).

Go to `target/arm-unknown-linux-musleabi/release` and copy `rpizero_rdc_rust`. Transfer it via FTP to your Raspberry Pi Zero W.

Change file permissions on your Raspberry Pi

```
chmod +x rpizero_rdc_rust
```

run the executable

```
./rpizero_rdc_rust
```
