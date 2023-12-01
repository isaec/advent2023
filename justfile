testwatch PKG BIN='part1 part2' ON='example':
  cargo watch -x 'test {{ON}} --package {{PKG}} --bin {{BIN}}'

test PKG BIN='part1 part2' ON='example':
  cargo test {{ON}} --package {{PKG}} --bin {{BIN}}

run PKG BIN:
  cargo run --package {{PKG}} --bin {{BIN}}