<div align="center">
  <h1>nadir</h1>
</div>

Run with `cargo make`.

## Prerequisites (assuming arch linux)

- cargo-make
- mold linker for dev and linux
- x86_64-w64-mingw32-gcc linker (from mingw-w64-gcc) for windows
- wasm-server-runner for running as wasm

```shell
    cargo install cargo-make wasm-server-runner
    sudo pacman -S mold mingw-w64-gcc
```
