"Completely analog" noise generator
===================================

A friends is very into analog synthesizers. I'm usually more on the digital side.
You can imagine the discussions we sometimes have about this ;-)

He recently baught [this drum synthesizer][1] that supposedly has "some digital
components" but he said that any passive module is "analog enough". What is a
passive module? Anything without a dedicated power supply, i.e. the only power
flowing into it is via its input signals.

Some time later, he mentioned that he doesn't know of any good analog noise generators.

Well.... let the trolling begin :-D

[1]: https://www.errorinstruments.com/a-85227938/bricky-format-standalone-box/po-passive-operator-drum-synth/


## Resources

- [Datasheet](https://www.wch-ic.com/downloads/CH32V203DS0_PDF.html)
- [Reference Manual](https://www.wch-ic.com/downloads/CH32FV2x_V3xRM_PDF.html)
- [Very useful information and example design](https://github.com/wuxx/nanoCH32V203)
- [Also useful](https://github.com/openwch/ch32v20x)
- [Programmer](https://github.com/ch32-rs/wchisp), eh, "analog configuration tool"

## Plan

- MCU
    - internal oscillator (for low power)
    - put into sleep until gate input is >=8 V
    - at least to ADC channels, fast enough to sample both inputs with >=50 kHz
    - CH32V2xx has two opamps, it seems
    - CRC32 unit might be useful for generating the noise (basically an LFSR).
    - DAC would be great but PWM is ok.
- power: diode rectifier, LDO, MLCC
    - maybe another MLCC on Vbat (if available), MLCC [seems to be good enough][cap-discharge]
- inputs:
    - 1x gate
        - comparator to wake up MCU (IC or something simple)
        - expect +-10 V / 1 kOhm at driver; add 100R on our side
    - 2x input: 10 kOhm and filter, to ADC
    - 1x output: amplify to +-10V if possible, 10 kOhm impedance should be ok for a passive module
- optional:
    - maybe an LED
        - sadly not WS2812 because of 1 mA quiescent current
        - Use high-efficiency LEDs and reduce the current a lot.
    - maybe a button, potentiometer or encoder
    - USB - for ~~programming~~ analog configuring and trolling
    - bootloader: pull PB8 high with VUSB (voltage divider! and solder jumper) or add a button
    - TRNG
    - one or more touch areas, "Theremin", also mix into the noise (if we don't have a TRNG)
        - The MCU even has dedicated hardware for that. Nice!
    - IMS :-D
        - [LIS2DH12TR](https://jlcpcb.com/partdetail/Stmicroelectronics-LIS2DH12TR/C110926)
          is cheap and only consumes some uA. Well... I guess that means yes.
        - There aren't any basic parts in the sensor category, so extended parts fee is more
          than component cost.
        - Typical sample rate ("ODR/2") seems to be 10/2 Hz or 50/2 Hz but can be as high as 5/2 kHz,
          so we could probably use this for input (tilt, then double-tap).
        - We could react to disturbance. We had lots of fun with that feature of a spring reverb :-)
- mechanical:
    - "analog noise generator with gate": shrink tube, put in the middle of a patch cable
    - "analog multiplier with gate": Eurorack module cover (3D printed or wood), glue PCB to its back, three jacks
    - 3D printed case in cable, with two parts that can be rotated to set a value

[cap-discharge]: https://www.robotroom.com/Capacitor-Self-Discharge-2.html

- How much power do we have available?
    - 10 V / 1 kOhm is 10 mA. As we use an LDO, that's also true at 3.3 V.
    - MCU need 0.5 mA to 16 mA depending on frequency, so let's stick to lower frequencies.
    - output will provide up to 1 mA at 10 kOhm impedance (plus what the amplifier needs).
    - We should expect to see up to 15 V continuous because 10 V is more like the minimum and
      many modules go up to 1..2 V below there supply of 12 V or 15 V.
- How much delay after the gate is asserted?
    - Startup time for oscillator alone is 2.5 ms. Internal HSI needs 10 us.
    - Let's expect a few ms overall and hope for the best.
    - It could be a lot less if gate rises slowly (we can start up before it reaches the
      threshold). In fact, that is our goal and that's why the threshold voltage is near
      the maximum.
- Any PCBs that we can use as reference or even copy?
    - https://github.com/metro94/FlappyBoard
    - I'm not sure which things on the board may be notices according to OHL but we will
      probably only copy the schematic anyway.
- Which LDO?
    - There don't seem to be any basic parts in category power.
    - More than 15 V (with some margin). Reasonable quiescent current.
    - 5 mA would be 60 mW.
    - [LR8321A-T33](https://www.lcsc.com/product-detail/Linear-Voltage-Regulators-LDO_LR-LR-LR8321A-T33_C5129918.html),
      [vendor page](https://www.lorysemi.com/product/80b8478f710f4a6b9008f729313d2654/a9d765514f14408ab45c6008f1d1d0b4):
      100 mA, 250 mW
- Pinout planning - assuming that we use the QFN28 package.
    - fixed:
        - 4x power
        - 2x crystal
        - 2x BOOT0, nRST  (BOOT0 can be an output after startup)
        - 2x USB
        - 2x SWD ?
        - 1x WKUP?
    - usable:
        - PA1..PA7 (ADC)
        - PB0..PB1 (ADC)
            - PA2..PB1 are also used for the opamps.
            - Touch uses the ADC pins.
            - Can all ADC inputs be connected to either of the two ADCs?
        - PA9
        - PA15
        - PB3..PB7
            - PB6 and PB7 are I2C1 (and PB5). PB6 is SCL, which can also be mapped to PB8. I2C is for IMS.
            - PB5 is `I2C_SMBA` (SMBus Alert), which we don't need.
        - summary: 9x ADC, 7x GPIO (including I2C)
    - needed:
        - 2x I2C -> PB6 and PB7
        - 1x wakeup+gate -> PA0 (WKUP and ADC0)
        - 2x input (ADC)
        - 2x positive and negative rail? (so we know our usable range)
            - wakeup is connected to unfiltered gate, i.e. not the same as current voltage in capacitor
        - 1..2x output (TIM)
        - 1x inverter for negative rail
        - 1x VUSB detect (BOOT0 cannot be used as an input)
            - Could be the same as e.g. a touch key (which is then not usable while VUSB is present).
        - sum so far: 10
        - 0..1x touch (ADC)
        - 4x LED ?
            - [charlieplexing](https://en.wikipedia.org/wiki/Charlieplexing)
            - 12 LEDs with 4 pins. Do we need a resistor per LED? Or half the resistor on each pin?
            - Alternative: Bidir matrix with 8 LEDs.
            - We can use BOOT0 pin but not as three-state.
        - 0x button ?
        - 1x poti ?

### TODO

- Can CH32V2xx run its USB bootloader from internal crystal or should we add an external one?
    - Well, let's add the external one and hopefully not use it (but it doesn't really need
      more power than HSI, to be honest).
- Low-power OpAmp for output or transistor amplifier?
    - Rail-to-rail would be useful. That should be easier with an integrated part.
    - There are lots of low power amplifiers, e.g. [LP358][LP358]. They are below 100 uA.
      Actually, we could tolerate a bit more than that. GBW is often not great but we amplify
      by 6x for audio so ~150 kHz should be fine (or less - depending on how much we filter
      our PWM anyway).
    - The cheapest LM321 on LCSC has 1.2 to 3 mA depending on VCC. That is a bit high but
      should work.
    - The cheapest LM358 on LSCS has below 1 mA even at 36 V. That's good. Output doesn't
      go up to positive rail even at very low current.
    - Cheapest with FET input on LCSC is TL072. It needs ~~a bit over 1 mA~~ per amplifier
      (so maybe try and use the 1x variant) and it goes to within 0.2 V of the rails at
      10 kOhm. Sounds good.
    - TL082 is similar but I just noticed that supply current is 1.4 to 2.8 mA per amplifier
      and that also goes for the TL072 (the figure from before was quiescent current).
    - Well, let's use one of them anyway. That's good enough.
    - We want TL072CDT because that's the only basic part in that category! Supply current is
      2.5 mA max but output swing is 1.5 V below the rails.
- Can we make a simple TRNG? What sources of entropy do we have?
    - [Infinity Noise](https://github.com/waywardgeek/infnoise) has good documentation but
      3 opamps is a bit much for us.
    - Ring oscillators, e.g. [like this](https://github.com/stnolting/neoTRNG). This is more
      for FPGA but we could use the oscillators of the MCU.
    - Tie unused ADC inputs to half VCC with high impedance.
    - Onchip temperature sensor.
    - Mix in the unique ID.
    - Store previous seed in RTC memory.
    - IMS.
- Connect VBAT?
    - It would be nice to have the RTC available but dedicated VBAT is only available on the
      larger packages.
    - Alternative: Not much connected to 3.3V supply, diode in series to LDO. Stop and standby
      don't use more current than RTC according to datasheet. Downside: It needs programming
      discipline and we have to power down anything that we connect to that supply (or
      power it through an IO pin).
- What do we do about negative voltages?
    - We will usually get a gate signal with 0 V and 10 V, I assume. And we have GND. We can add a
      negative voltage rail but it won't usually be charged.
    - That's not so great because our opamp won't be so great near the negative rail, which will
      be GND in our case.
    - Simple switched capacitor voltage inverter to at least have -2.9 V? -> Probably a good idea.
- How much capacity for power buffer? How to connect it?
    - Long enough to provide useful time for RTC. MCU and IMS will draw some tens of uA.
        - Starting at 3.3 V. Normal operating range is down to 2.4 V. RTC can work down to
          1.8 V (when powered through VBAT).
        - Example: 1 uF = 50uA*t/(3.3V-2.4V) => t = 0.018 sec => not so great
    - Long enough to switch MCU to low-power mode, e.g. a few ms. Maybe long enough to continue
      working for a few ms to compensate startup delay.
        - Let's plan with 5 mA and 10 ms.
        - Voltage can go from 10 V to 5 V but that will be noticed in the output signal, of course.
        - That would be 10 uF. That is reasonable.
        - However... how long would it take to charge this? tau = 100 ms. Not great.
    - Another issue: We cannot set our gate threshold to 8 V because we will see less voltage
      while we are drawing power or even charging the capacitor.
        - If we draw 5 mA, we will see half the voltage. Not only for the gate but also for the
          capacitor. D'oh!
        - So... it would be good to limit ourselves to 3 mA and not much capacity.
        - The gate should have a large hysteresis.
    - Options?
        - Supply for MCU by DCDC (up to 3x more efficient).
        - Use less power for the MCU. It needs only 0.5 mA at low frequencies.
        - Voltage doubler (and maybe a voltage inverter from the higher voltage instead of MCU pin).
        - Generate output voltage from MCU pin (0 to 3.3V range, only passive filter).
        - Generate output voltage with h-bridge plus passive filter (power consumption to be determined).
            - Example: 10 kOhm plus 5 nF (and maybe 1 kOhm to output). tau=50 us (20 kHz). Current is 1 mA
              at 100 kHz according to Falstad.
            - Even when we add a transistor as a voltage follower, we will need the 1 mA if we want
              10 kOhm output impedance.
            - We may want to add a 2nd filter stage. The one stage has a ripple of +-300 mV!
        - Generate digital signal without much of a filter (only good for hard noise).
            - We should have this as an option.
        - Use an actual low-power opamp.
        - Hope that the opamp is closer to its quiescent current than supply current because our
          signals are slow (but that is still more than 2 mA for the dual opamp).

    - TODO: Simulation.
        - 10V -> 1k -> Diode -> 1 uF -> VCC
        - Voltage doubler. Voltage inverter. With actual MOSFETs.
        - PWM signal -> half bridge -> filter
        - Turn that 10V on and off.



[LP358]: https://www.ti.com/lit/ds/symlink/lp358.pdf?ts=1717468161669

