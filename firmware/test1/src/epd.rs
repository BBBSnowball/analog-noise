use core::convert::Infallible;

use crate::hal::{
    gpio::*,
    gpio::gpioc::*,
    gpio::gpiof::*,
    spi,
    delay::Delay,
};
use embedded_hal::spi::SpiDevice;
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

type RESET<MODE> = PF0<MODE>;
type DC<MODE> = PF1<MODE>;
type BUSY<MODE> = PC14<MODE>;

//NOTE Arguments are not consistent between Ssd1680 and DisplayInterface, so we have to swap DC and RESET here.
type EPD<SPI> = Ssd1680<SPI, BUSY<Input<Floating>>, DC<Output<PushPull>>, RESET<Output<PushPull>>>;

pub struct EpaperDisplay<SPI> {
    epd: EPD<SPI>,
}

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

impl<SPI> EpaperDisplay<SPI>
where
    SPI: SpiDevice
{
    pub fn new(
        spi: SPI,
        reset: RESET<Input<Floating>>,
        dc: DC<Input<Floating>>,
        busy: BUSY<Input<Floating>>,
        delay: &mut Delay,
    ) -> Self {
        let (reset, dc, busy) = cortex_m::interrupt::free(move |cs| {
            (
                reset.into_push_pull_output(cs),
                dc.into_push_pull_output(cs),
                busy.into_floating_input(cs),
            )
        });

        let epd = Ssd1680::new(spi, busy, dc, reset, delay).unwrap();

        EpaperDisplay {
            epd,
        }
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

pub fn test_epd<SPI>(epd: &mut EpaperDisplay<SPI>, delay: &mut Delay) -> Result<(), Error>
where
    SPI: SpiDevice
{
    epd.epd.clear_bw_frame()?;
    epd.epd.clear_red_frame()?;

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
    epd.epd.update_bw_frame(display_bw.buffer())?;

    epd.epd.display_frame(delay)?;

    Ok(())
}
