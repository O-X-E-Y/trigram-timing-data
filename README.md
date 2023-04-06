## Usage

```sh
cargo run --release <path>
```
```sh
cargo run --release ./data/dvorak_1k.json
```
alternatively, you can
```sh
cargo build --release
cd target/release
./trigram-timing-data ../../data/dvorak_1k.json
```