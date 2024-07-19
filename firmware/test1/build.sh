set -xe
ELF=target/thumbv6m-none-eabi/debug/analog-noise-test1
cargo build
cargo objdump -- -xd >"$ELF.map"
cargo nm -- --print-size --size-sort --demangle --radix=d | tail
cargo size -- -d
cargo objcopy -- -O binary "$ELF.bin"
