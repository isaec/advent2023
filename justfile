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
  fastmod '^members = \[((?:.|\n)*?)\]' 'members = [${1}    "{{PKG}}"]' -- Cargo.toml

dev PKG BIN:
  # watch the example test until it passes then run the binary
  cargo watch -x 'test example --package {{PKG}} --bin {{BIN}}' -s 'cargo run --package {{PKG}} --bin {{BIN}}'

test_libs:
  cargo test --all --lib

testwatch_libs:
  cargo watch -x 'test --all --lib'