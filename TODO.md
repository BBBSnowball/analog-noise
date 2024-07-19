# PCB

These have not been implemented in what is now v1 of the board, so they probably never will be implemented ;-)

- more testpoints
    - SMD testpoints for unused pins of MCU
    - DAC
    - Vsense
    - WKUP
    - 3V3 for poti
- doch die L Variante vom MCU? -> ne, aktuell nur im großen Package verfügbar

# Potential Errata

1. We couldn't flash any firmware with ST bootloader. Is that an error in the hardware..?
    - The bootloader is enumerated and dfu-util can query its memory ranges, so most of USB is working.
      Also, USB CDC example is working fine. DFU hangs when erasing flash.
    - The normal dfu-util should support STM32F0 if we pass it an address with `-s` (which we do).
    - Some boards have 27R in USB data lines but many have 0R, so our lack of them should be fine.
    - Bluepill has pullup on one of the USB data lines but enumeration works, so this should be fine.
    - We found some mention that HSE is used for the bootloader and we don't have any external crystal.
      Again, enumeration should not work if that was the issue and schematic for STM32F072B-DISCO also
      omits the crystal.
    - We are out of ideas here, for now.
2. White LED is brighter than the others.
