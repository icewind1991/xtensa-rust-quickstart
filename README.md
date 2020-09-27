
# Rust on the xtensa architecture

Need help? Join the esp-rs room on matrix, https://matrix.to/#/#esp-rs:matrix.org.

## Building

## Setting up the compiler

- setup the [xtensa rust](https://github.com/MabezDev/rust-xtensa) compiler.

```
$ git clone https://github.com/MabezDev/rust-xtensa
$ cd rust-xtensa
$ ./configure --experimental-targets=Xtens
$ ./x.py build
```

- link the custom rust build into rustup

```
$ rustup toolchain link xtensa /path/to/rust-xtensa/build/x86_64-unknown-linux-gnu/stage1
```

- install the xtensa-lx106-elf toolchain from the [espressif web site](https://docs.espressif.com/projects/esp8266-rtos-sdk/en/latest/get-started/linux-setup.html).

```
$ mkdir ~/esp
$ tar -xzf ~/Downloads/xtensa-lx106-elf-linux64-1.22.0-100-ge567ec7-5.2.0.tar.gz -C ~/esp
$ PATH="$PATH:$HOME/esp/xtensa-lx106-elf/bin"
```

- install cargo-espflash

```
$ cargo install cargo-espflash
```

## Build and flash

```
$ cargo espflash --release /dev/ttyUSB0
```


## Resources

- The [esp-rs](https://github.com/esp-rs) organization has been formed to develop runtime, pac and hal crates for the esp32 and eventually 

## FAQ

- `LLVM ERROR: Error while trying to spill A10 from class AR` - try building in release mode
