set -xe
cargo build
#NOTE This won't work because it will jump to ST bootloader.
probe-rs run --chip STM32F072CBTx target/thumbv6m-none-eabi/debug/analog-noise-test1
