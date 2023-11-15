## Usage

```
cargo run --release -- <path> [path, ...]
```
```
cargo run --release -- ./data/dvorak_1k.json
```
alternatively, you can
```sh
cargo build --release
cd target/release
./trigram-timing-data -- ../../data/dvorak_1k.json
```