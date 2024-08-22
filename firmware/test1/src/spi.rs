use core::convert::Infallible;

use crate::hal::{
    gpio::*,
    gpio::gpiob::*,
    pac,
    prelude::*,
    rcc::Rcc,
    spi,
    spi::{Mode, Phase, Polarity, EightBit},
};
use embedded_hal::{
    digital::{ErrorType as ErrorTypeDigital, OutputPin},
    spi::{ErrorType as ErrorTypeSpi, SpiDevice}
};
use embedded_hal_bus::{spi::{AtomicDevice, AtomicError, NoDelay}, util::AtomicCell};

type SCK<MODE> = PB13<MODE>;
type MISO<MODE> = PB14<MODE>;
type MOSI<MODE> = PB15<MODE>;

type SPI = spi::Spi<pac::SPI2, SCK<Alternate<AF0>>, MISO<Alternate<AF0>>, MOSI<Alternate<AF0>>, EightBit>;

#[derive(Debug)]
pub enum Error {
    SpiError(spi::Error),
    Timeout,
}

impl From<spi::Error> for Error {
    fn from(error: spi::Error) -> Self {
        Error::SpiError(error)
    }
}

pub struct TimesharedSpi {
    pub spi: AtomicCell<SPI>,
}

pub type Spi<'a, CS> = AtomicDevice<'a, SPI, CS, NoDelay>;

impl TimesharedSpi {
    pub fn new(
        pb13: SCK<Input<Floating>>, pb14: MISO<Input<Floating>>, pb15: MOSI<Input<Floating>>,
        spi: pac::SPI2, rcc: &mut Rcc
    ) -> Self {
        // IMS wants mode 3 and I think this is already the right setting here.
        const MODE: Mode = Mode {
            polarity: Polarity::IdleHigh,
            phase: Phase::CaptureOnSecondTransition,
        };
    
        // Configure pins for SPI
        let (sck, miso, mosi) = cortex_m::interrupt::free(move |cs| {
            (
                pb13.into_alternate_af0(cs),
                pb14.into_alternate_af0(cs),
                pb15.into_alternate_af0(cs),
            )
        });
    
        // Configure SPI with 100kHz rate
        //FIXME IMS should support more. This is copied from an example.
        let spi = spi::Spi::spi2(spi, (sck, miso, mosi), MODE, 100_000.hz(), rcc);

        Self { spi: AtomicCell::new(spi) }
    }

    pub fn try_make_device<'a, CS>(&'a self, chip_select: CS) -> Result<Spi<'a, CS>, CS::Error> where CS: OutputPin {
        AtomicDevice::new_no_delay(&self.spi, chip_select)
    }

    pub fn make_device<'a, CS>(&'a self, chip_select: CS) -> Spi<'a, CS>
    where CS: OutputPin, CS: ErrorTypeDigital<Error = Infallible>
    {
        self.try_make_device(chip_select).unwrap()
    }
}

pub struct RepeatWhenBusy<'a, CS> {
    spi: Spi<'a, CS>
}

impl<'a, CS> RepeatWhenBusy<'a, CS> {
    pub fn new(spi: Spi<'a, CS>) -> Self {
        Self { spi }
    }
}

impl<'a, CS> ErrorTypeSpi for RepeatWhenBusy<'a, CS>
where CS: OutputPin
{
    type Error = <Spi<'a, CS> as ErrorTypeSpi>::Error;
}

impl<'a, CS, E> SpiDevice for RepeatWhenBusy<'a, CS>
where
    Spi<'a, CS>: SpiDevice,
    Spi<'a, CS>: ErrorTypeSpi<Error = AtomicError<E>>,
    CS: OutputPin,
    E: embedded_hal::spi::Error,
{
    fn transaction(&mut self, operations: &mut [embedded_hal::spi::Operation<'_, u8>]) -> Result<(), Self::Error> {
        // We run each operation on its own because we need to know how many
        // we have to repeat in case of busy and we don't want to block for
        // too long.
        for i in 0..operations.len() {
            loop {
                match self.spi.transaction(&mut operations[i..i]) {
                    Ok(()) => break,
                    Err(AtomicError::Busy) => (),
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(())
    }
}
