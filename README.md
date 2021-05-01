# Delusion soft renderer -- Minimum Dependency

毕业设计。

## Backend Crates

- nalgebra
- image
- minifb

## Build

```
cd ./Delusion
cargo build
cargo run --release [objpath/prefix]
```

## Performance

Average frame time: (With resolution 800x800, WeirdShader)

- Enable 4xMSAA: 17fps / 57ms
- Disable MSAA : 76fps / 13ms