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

Average frame time: 

- Enable 4xMSAA: 27fps / 36ms
- Disable MSAA : 56fps / 17ms