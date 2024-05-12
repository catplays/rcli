
注意：
参数简写使用“-”, 全写使用"--"

# base64
## 默认格式
加密：
cargo run -- base64 encode 
输入数据后： ctrl+D

解密：
cargo run -- base64 decode
输入数据后： ctrl+D

## 制定格式和文件
cargo run -- base64 encode --format urlsafe -i Cargo.toml > tmp.b64

cargo run -- base64 decode --format urlsafe -i  tmp.b64

# sign
cargo run -- signature sign -k fixtures/blake3.txt
输入数据后： ctrl+D

cargo run -- signature verify --key fixtures/blake3.txt --sig 粘贴数据

## generate

cargo run -- signature generate -o fixtures --format ed25519

