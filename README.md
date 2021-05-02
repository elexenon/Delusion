# Delusion soft renderer -- Minimum Dependency

毕业设计。

## Backend Crates

- nalgebra
- image
- minifb

## Environment

macOS 11.2.3
LLVM 12.0.0

## Build

```
cd ./Delusion
cargo build
cargo run --release [objpath/prefix]
```

## Performance

i7 6700hq 2.6Ghz

Average frame time: (With resolution 800x800, WeirdShader)

- Enable 4xMSAA: 17fps / 57ms
- Disable MSAA : 83fps / 12ms