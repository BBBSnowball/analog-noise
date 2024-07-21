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
3. Touch sensing (TSC) needs one capacitor per group.
    - Workaround: We expected it to be similar to touch sensing on AVR and it seems to be possible
      to do it like that: https://github.com/arpruss/ADCTouchSensor
    - Downside of that workaround: It is probably less sensitive and we would rather use the ADC for
      the input signals.
    - If we fix this for v2, we have many more pins that we can use (not only ADC) but we don't have
      any free pins for the sampling capacitors in the existing groups, i.e. pinout will have to change.
    - We can add capacitors to PA0 (challenging) and PA7 (easy) on the v1 board and then use TOUCH1 and TOUCH3.
    - Can we compare TOUCH1 to TOUCH2, with both of them floating without any capacitor?
      -> No, resulting value is just 2 or 3.
4. PWM for all LEDs.
    - Use PB5 instead of PB2.
    - PB3 uses TIM2, which is not available for PWM in the hal, for some reason.
5. R3 should be 1k instead of 10 k but it is only used for testing.
6. Power consumption is higher than expected.
    - 4-5 mA with opamp disabled and MCU in `wfi()` sleep mode.
7. Glitch on DAC at startup.
    - Both DAC outputs glitch high for a short time (< 5 us) when starting up.
    - TODO: This was without the opamp. How does it behave with the opamp?

# Test Hardware

- Use interrupt for IMS.
- Test single and double click with IMS.
- PWM for the LEDs if possible.
- Add capactors for TSC to PA0 and PA7.
    - [This page](https://wiki.st.com/stm32mcu/wiki/Introduction_to_touch_sensing_with_STM32) says typical
      range is 22 nF to 220 nF.
- Test `VUSB_DET`.
- Test `VIN_SENSE`, `V+_SENSE`, IN1, IN2
- Test comparator for `VIN_SENSE`
- Test DAC1 and DAC2
- Implement low-power mode and test with limit input power.
- Use CRC unit to generate noise.
- Test with EPD.
- First application:
    - Generate noise.
    - Disable output when gate VIN is gone and go to very low-power mode.
    - Measure startup delay and maybe compensate when turning off.
    - Use IMS do control color of noise (and maybe touch so it changes when touched).
