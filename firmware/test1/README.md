```
rustup target add thumbv6m-none-eabi
#cargo install cargo-binutils cargo-bloat
cargo build
```

SVD file is from:
https://github.com/stm32-rs/stm32-rs/blob/master/svd/vendor/en.stm32f0_svd.zip

## Debugging

- use Raspberry Pico as the debug probe: SWCLK on pin 2, SWDIO on pin 3
    - There is a GND next to pin 2 so this exactly matches the order on our pin header - which is a lucky accident.
    - We couldn't find any official documentation on the pinout (the linked guides are for Raspberry Pi, not Pico)
      but it is [in the source code](https://github.com/raspberrypi/debugprobe/blob/master/include/board_pico_config.h).
    - Firmware is available [in releases of that project](https://github.com/raspberrypi/debugprobe/releases/tag/debugprobe-v2.0.1):
      `picotool load ~/Downloads/debugprobe_on_pico.uf2`
- start VS Code under `nix develop .#rust`
- install Cortex-Debug extension in VS Code
- start the debug configuration from `launch.json`
