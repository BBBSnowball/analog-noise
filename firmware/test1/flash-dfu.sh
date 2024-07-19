set -xe
ELF=target/thumbv6m-none-eabi/debug/analog-noise-test1
./build.sh
dfu-util -s 0x08000000:leave -D "$ELF.bin" --alt 0 -d 0483:df11
