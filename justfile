testwatch PKG BIN='part1 part2' ON='example':
  cargo watch -x 'test {{ON}} --package {{PKG}} --bin {{BIN}}'

test PKG BIN='part1 part2' ON='example':
  cargo test {{ON}} --package {{PKG}} --bin {{BIN}}

run PKG BIN:
  cargo run --package {{PKG}} --bin {{BIN}}

new PKG:
  cp -vr template {{PKG}}
  fastmod 'template' '{{PKG}}' -- {{PKG}}/Cargo.toml
  fastmod '^members = \[((?:.|\n)*?)\]' 'members = [${1}    "{{PKG}}"]' -- Cargo.toml

test_libs:
  cargo test --all --lib