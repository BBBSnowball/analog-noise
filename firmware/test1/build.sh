set -xe
#ELF=target/thumbv6m-none-eabi/debug/analog-noise-test1
ELF=target/thumbv6m-none-eabi/debug/main
#export RUSTC_LOG=rustc_codegen_ssa::back::link=info
cargo build
cargo objdump -- -xd >"$ELF.map"
cargo nm -- --print-size --size-sort --demangle --radix=d | tail
cargo size -- -d
cargo objcopy -- -O binary "$ELF.bin"
