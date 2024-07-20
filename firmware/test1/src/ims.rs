use crate::hal::{
    gpio::*,
    gpio::gpiob::*,
    pac,
    prelude::*,
    rcc::Rcc,
    spi,
    spi::Spi,
    spi::{Mode, Phase, Polarity, EightBit},
};
use core::{convert::Infallible, fmt::Write};
use cortex_m_semihosting::hio::HostStream;

type CS<MODE> = PB12<MODE>;
type SCK<MODE> = PB13<MODE>;
type MISO<MODE> = PB14<MODE>;
type MOSI<MODE> = PB15<MODE>;
type INT<MODE> = gpioa::PA9<MODE>;

pub struct IMS {
    cs: CS<Output<PushPull>>,
    int: INT<Input<PullUp>>,
    spi: Spi<pac::SPI2, SCK<Alternate<AF0>>, MISO<Alternate<AF0>>, MOSI<Alternate<AF0>>, EightBit>,
}

#[derive(Debug)]
pub enum Error {
    SpiError(spi::Error),
    PinError(Infallible),
    FormatError(core::fmt::Error),
    WrongId,
}

impl From<spi::Error> for Error {
    fn from(error: spi::Error) -> Self {
        Error::SpiError(error)
    }
}

impl From<core::fmt::Error> for Error {
    fn from(error: core::fmt::Error) -> Self {
        Error::FormatError(error)
    }
}

impl IMS {
    pub fn new(pb12: CS<Input<Floating>>, pa9: INT<Input<Floating>>,
        pb13: SCK<Input<Floating>>, pb14: MISO<Input<Floating>>, pb15: MOSI<Input<Floating>>,
        spi: pac::SPI2, rcc: &mut Rcc
    ) -> Self {
        // IMS wants mode 3 and I think this is already the right setting here.
        const MODE: Mode = Mode {
            polarity: Polarity::IdleHigh,
            phase: Phase::CaptureOnSecondTransition,
        };
    
        // Configure pins for SPI
        let (mut cs, int, sck, miso, mosi) = cortex_m::interrupt::free(move |cs| {
            (
                pb12.into_push_pull_output(cs),
                pa9.into_pull_up_input(cs),
                pb13.into_alternate_af0(cs),
                pb14.into_alternate_af0(cs),
                pb15.into_alternate_af0(cs),
            )
        });
        cs.set_high().unwrap();
    
        // Configure SPI with 100kHz rate
        //FIXME IMS should support more. This is copied from an example.
        let spi = Spi::spi2(spi, (sck, miso, mosi), MODE, 100_000.hz(), rcc);

        IMS {
            cs,
            int,
            spi,
        }
    }

    #[allow(non_upper_case_globals)]
    const READ_nWRITE : u8 = 0x80;
    const ADDRESS_AUTO_INCREMENT : u8 = 0x40;

    fn with_chip_selected<F, R>(self: &mut Self, func: F) -> Result<R, Error>
        where F: FnOnce(&mut Self) -> Result<R, Error>
    {
        self.cs.set_low().map_err(Error::PinError)?;
        let result = func(self);
        self.cs.set_high().map_err(Error::PinError)?;
        result
    }

    fn transfer(self: &mut Self, bytes: &mut [u8]) -> Result<(), Error> {
        self.with_chip_selected(|self2| {
            self2.spi.transfer(bytes)?;
            Ok(())
        })
    }

    pub fn read(self: &mut Self, address: u8) -> Result<u8, Error> {
        let mut bytes = [Self::READ_nWRITE | (address & 0x3f), 0];
        self.transfer(&mut bytes)?;
        Ok(bytes[1])
    }

    pub fn write(self: &mut Self, address: u8, value: u8) -> Result<(), Error> {
        let mut bytes = [(address & 0x3f), value];
        self.transfer(&mut bytes)
    }

    pub fn write_auto_inc(self: &mut Self, address: u8, values: &mut [u8]) -> Result<(), Error> {
        self.with_chip_selected(|self2| {
            let mut bytes = [Self::ADDRESS_AUTO_INCREMENT | (address & 0x3f)];
            self2.spi.transfer(&mut bytes)?;
            self2.spi.transfer(values)?;
            Ok(())
        })
    }

    pub fn read_auto_inc(self: &mut Self, address: u8, values: &mut [u8]) -> Result<(), Error> {
        self.with_chip_selected(|self2| {
            let mut bytes = [Self::READ_nWRITE | Self::ADDRESS_AUTO_INCREMENT | (address & 0x3f)];
            self2.spi.transfer(&mut bytes)?;
            self2.spi.transfer(values)?;
            Ok(())
        })
    }
}

pub fn test_ims(mut stdout: HostStream, ims: &mut IMS) -> Result<(), Error> {
    let who_am_i = ims.read(0x0f);
    if let Ok(who_am_i) = who_am_i {
        write!(stdout, "IMS, WHO_AM_I: {:x}\r\n", who_am_i)?;
        if who_am_i != 0x33 {
            write!(stdout, "IMS: ERROR: ID is not as expected\r\n")?;
            return Err(Error::WrongId)
        }
    } else {
        write!(stdout, "IMS, WHO_AM_I: error\r\n")?;
        return Err(Error::WrongId)
    }

    ims.write_auto_inc(0x1e, &mut[
        // CTRL_REG0 (1e): disable pullup on DO pin because that should use less power
        0x90,
        // TEMP_CFG_REG (1f): enable temperature sensor
        0xc0,
        // CTRL_REG1 (20): 100 Hz, low-power, all axes
        0x5f,
        // CTRL_REG2 (21): no high-pass filter
        0x00,
        // CTRL_REG3 (22): interrupt for click and FIFO watermark
        0x84,
        // CTRL_REG4 (23): set BDU (required for temperature sensor, says datasheet)
        0x80,
        // CTRL_REG5 (24): keep defaults
        0x00,
        // CTRL_REG6 (25): keep defaults
        0x00,
    ])?;

    // read REFERENCE (26) because datasheet suggests this when switching modes
    let _ = ims.read(0x26)?;

    for _ in 0..4 {
        // temp[1] should change by ~1 for every 1 K change in temperatur
        // (and temp[0] will be 0 because of low-power mode)
        let mut temp = [0; 2];
        ims.read_auto_inc(0x0c, &mut temp)?;
        write!(stdout, "TEMP: {:02x}{:02x}\r\n", temp[1], temp[0])?;

        let mut data = [0; 9];
        ims.read_auto_inc(0x27, &mut data)?;
        write!(stdout, "STATUS_REG: {:02x}\r\n", data[0])?;
        write!(stdout, "OUT_X: {:02x}{:02x}\r\n", data[2], data[1])?;
        write!(stdout, "OUT_Y: {:02x}{:02x}\r\n", data[4], data[3])?;
        write!(stdout, "OUT_Z: {:02x}{:02x}\r\n", data[6], data[5])?;
        write!(stdout, "FIFO REGS: {:02x}, {:02x}\r\n", data[7], data[8])?;
    }

    Ok(())
}
