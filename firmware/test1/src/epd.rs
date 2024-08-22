use core::convert::Infallible;

use crate::hal::{
    gpio::*,
    gpio::gpiob::*,
    gpio::gpioc::*,
    gpio::gpiof::*,
    pac,
    prelude::*,
    rcc::Rcc,
    spi,
    spi::Spi,
    spi::{Mode, Phase, Polarity, EightBit},
    delay::Delay,
};
use rtt_target::rprintln;
use ssd1680::{driver::DisplayError, prelude::*};
use ssd1680::color::Black;
use embedded_graphics::{
    prelude::*,
    primitives::{Circle, Rectangle},
    mono_font::{ascii::FONT_5X8, MonoTextStyle},
    pixelcolor::BinaryColor,
    primitives::PrimitiveStyle,
    text::{Alignment, Text, TextStyleBuilder}
};

type CS<MODE> = PC15<MODE>;
type SCK<MODE> = PB13<MODE>;
type MISO<MODE> = PB14<MODE>;
type MOSI<MODE> = PB15<MODE>;
type RESET<MODE> = PF0<MODE>;
type DC<MODE> = PF1<MODE>;
type BUSY<MODE> = PC14<MODE>;

type SPI = Spi<pac::SPI2, SCK<Alternate<AF0>>, MISO<Alternate<AF0>>, MOSI<Alternate<AF0>>, EightBit>;
//type EPD = Ssd1680<SPI, BUSY<Input<Floating>>, RESET<Output<PushPull>>, DC<Output<PushPull>>>;
//NOTE Arguments are not consistent between Ssd1680 and DisplayInterface, so we have to swap DC and RESET here.
type EPD = Ssd1680<SPI, BUSY<Input<Floating>>, DC<Output<PushPull>>, RESET<Output<PushPull>>>;

pub struct EpaperDisplay {
    cs: CS<Output<PushPull>>,
    //reset: RESET<Output<PushPull>>,
    //dc: DC<Output<PushPull>>,
    //busy: BUSY<Input<Floating>>,
    //spi: SPI,
    epd: EPD,
}

// Directions on the PCB:
// Z is positive when PCB is flat on the table with USB port on top.
// X is positive when PCB stands on the small edge with inputs+USB pointing up.
// Z is positive when PCB stands on the long edge with USB pointing down.

#[derive(Debug)]
pub enum Error {
    SpiError(spi::Error),
    PinError(Infallible),
    DisplayError(DisplayError),
}

impl From<spi::Error> for Error {
    fn from(error: spi::Error) -> Self {
        Error::SpiError(error)
    }
}

impl From<DisplayError> for Error {
    fn from(error: DisplayError) -> Self {
        Error::DisplayError(error)
    }
}

impl EpaperDisplay {
    pub fn new(
        cs: CS<Input<Floating>>,
        reset: RESET<Input<Floating>>,
        dc: DC<Input<Floating>>,
        busy: BUSY<Input<Floating>>,
        pb13: SCK<Input<Floating>>, pb14: MISO<Input<Floating>>, pb15: MOSI<Input<Floating>>,
        spi: pac::SPI2, rcc: &mut Rcc,
        delay: &mut Delay,
    ) -> Self {
        const MODE: Mode = Mode {
            polarity: Polarity::IdleHigh,
            phase: Phase::CaptureOnSecondTransition,
        };

        // Configure pins for SPI
        let cspin = cs;
        let (mut cs, reset, dc, busy, sck, miso, mosi) = cortex_m::interrupt::free(move |cs| {
            (
                cspin.into_push_pull_output(cs),
                reset.into_push_pull_output(cs),
                dc.into_push_pull_output(cs),
                busy.into_floating_input(cs),
                pb13.into_alternate_af0(cs),
                pb14.into_alternate_af0(cs),
                pb15.into_alternate_af0(cs),
            )
        });
        cs.set_high().unwrap();

        let spi = Spi::spi2(spi, (sck, miso, mosi), MODE, 100_000.hz(), rcc);

        let epd = Ssd1680::new(spi, busy, dc, reset, delay).unwrap();

        EpaperDisplay {
            cs,
            epd,
        }
    }

    fn with_chip_selected<F, R>(self: &mut Self, func: F) -> Result<R, Error>
        where F: FnOnce(&mut Self) -> Result<R, Error>
    {
        self.cs.set_low().map_err(Error::PinError)?;
        let result = func(self);
        self.cs.set_high().map_err(Error::PinError)?;
        result
    }
}

fn draw_text(display: &mut Display2in13, text: &str, position: Point) -> Result<(), Error> {
    let style = MonoTextStyle::new(&FONT_5X8, 
        // text should be black
        BinaryColor::Off);
    let builder = TextStyleBuilder::new()
        .alignment(Alignment::Center)
        .build();
    Text::with_text_style(text, position, style, builder)
        .draw(display)?;
    Ok(())
}

pub fn test_epd(epd: &mut EpaperDisplay, delay: &mut Delay) -> Result<(), Error> {
    epd.with_chip_selected(|self2| {
        self2.epd.clear_bw_frame()?;
        self2.epd.clear_red_frame()?;
        Ok(())
    })?;

    let mut display_bw = Display2in13::bw();

    display_bw.set_rotation(DisplayRotation::Rotate0);

    Rectangle::new(Point::new(60, 60), Size::new(40, 40))
        .into_styled(PrimitiveStyle::with_fill(Black))
        .draw(&mut display_bw)
        .unwrap();

    Circle::new(Point::new(100, 60), 30)
        .into_styled(PrimitiveStyle::with_stroke(Black, 2))
        .draw(&mut display_bw)
        .unwrap();

    draw_text(&mut display_bw, "Just a test", Point::new(0, 35))?;

    rprintln!("Send bw frame to display");
    epd.with_chip_selected(|self2| {
        self2.epd.update_bw_frame(display_bw.buffer())?;

        self2.epd.display_frame(delay)?;

        Ok(())
    })?;

    Ok(())
}
