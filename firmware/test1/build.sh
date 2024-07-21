set -xe
#ELF=target/thumbv6m-none-eabi/debug/analog-noise-test1
#export RUSTC_LOG=rustc_codegen_ssa::back::link=info
cargo build
for prog in main main2 ; do
    ELF=target/thumbv6m-none-eabi/debug/$prog
    cargo objdump --bin $prog -- -xd >"$ELF.map"
    cargo nm --bin $prog -- --print-size --size-sort --demangle --radix=d | tail -n 5 | sed 's/^\w* /\t\t/'
    cargo size --bin $prog -- -d
    cargo objcopy --bin $prog -- -O binary "$ELF.bin"
done
