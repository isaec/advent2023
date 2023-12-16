testwatch PKG BIN='part1 part2' ON='example':
  cargo watch -x 'test {{ON}} --package {{PKG}} --bin {{BIN}}'

test PKG BIN='part1 part2' ON='example':
  cargo test {{ON}} --package {{PKG}} --bin {{BIN}}

run PKG BIN:
  cargo run --package {{PKG}} --bin {{BIN}}

run_release PKG BIN:
  cargo run --release --package {{PKG}} --bin {{BIN}}

new PKG:
  cp -vr template {{PKG}}
  fastmod 'template' '{{PKG}}' -- {{PKG}}/Cargo.toml
  fastmod '^members = \[(\n)((?:.|\n)*?)\]' 'members = [${1}${2}    "{{PKG}}",${1}]' -- Cargo.toml

dev PKG BIN:
  # watch the example test until it passes then run the binary
  cargo watch -x 'test example --package {{PKG}} --bin {{BIN}} -- --nocapture' -s 'cargo run --package {{PKG}} --bin {{BIN}}'

dev_release PKG BIN:
  # watch the example test until it passes then run the binary
  cargo watch -x 'test --release example --package {{PKG}} --bin {{BIN}} -- --nocapture' -s 'cargo run --release --package {{PKG}} --bin {{BIN}}'

bench PKG BIN:
  cargo build --release --package {{PKG}} --bin {{BIN}}
  hyperfine -N 'target/release/{{BIN}}'

test_libs:
  cargo test --all --lib

testwatch_libs:
  cargo watch -x 'test --all --lib'

clippy PKG:
  cargo clippy --package {{PKG}} -- -W clippy::pedantic

clippy_fix PKG:
  cargo clippy --package {{PKG}} --fix -- -W clippy::pedantic

clippy_all:
  cargo clippy --all -- -W clippy::pedantic

clippy_fix_all:
  cargo clippy --all --fix -- -W clippy::pedantic