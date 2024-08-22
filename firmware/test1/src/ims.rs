use crate::hal::{
    gpio::*,
    gpio::gpiob::*,
    pac,
    pac::interrupt,
    spi,
};
use core::{cell::RefCell, convert::Infallible, ops::DerefMut};
use cortex_m::interrupt::Mutex;
use embedded_hal::spi::SpiDevice;
use embedded_hal_bus::spi::{AtomicError, DeviceError};
use rtt_target::{UpChannel, rprintln};

type CS<MODE> = PB12<MODE>;
type INT<MODE> = gpioa::PA9<MODE>;

type SPI<'a> = crate::spi::Spi<'a, CS<Output<PushPull>>>;

pub struct IMS<'a> {
    //cs: CS<Output<PushPull>>,
    //int: INT<Input<Floating>>,
    spi: SPI<'a>,
}

// Directions on the PCB:
// Z is positive when PCB is flat on the table with USB port on top.
// X is positive when PCB stands on the small edge with inputs+USB pointing up.
// Z is positive when PCB stands on the long edge with USB pointing down.

#[derive(Debug)]
pub enum Error {
    SpiError(spi::Error),
    PinError(Infallible),
    WrongId,
    Busy,
}

impl From<spi::Error> for Error {
    fn from(error: spi::Error) -> Self {
        Error::SpiError(error)
    }
}

impl From<AtomicError<DeviceError<spi::Error, Infallible>>> for Error {
    fn from(value: AtomicError<DeviceError<spi::Error, Infallible>>) -> Self {
        match value {
            AtomicError::Busy => Error::Busy,
            AtomicError::Other(DeviceError::Cs(e)) => Error::PinError(e),
            AtomicError::Other(DeviceError::Spi(e)) => Error::SpiError(e),
        }
    }
}

impl<'a> IMS<'a> {
    pub fn new(spi: SPI<'a>, pa9: INT<Input<Floating>>) -> Self {
        let int = cortex_m::interrupt::free(move |cs| {
            (
                pa9.into_floating_input(cs),
            )
        });

        let _ = int;

        IMS {
            //int,
            spi,
        }
    }

    #[allow(non_upper_case_globals)]
    const READ_nWRITE : u8 = 0x80;
    const ADDRESS_AUTO_INCREMENT : u8 = 0x40;

    fn with_chip_selected<F, R>(self: &mut Self, func: F) -> Result<R, Error>
        where F: FnOnce(&mut Self) -> Result<R, Error>
    {
        func(self)
    }

    fn transfer(self: &mut Self, bytes: &mut [u8]) -> Result<(), Error> {
        self.with_chip_selected(|self2| {
            self2.spi.transfer_in_place(bytes)?;
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
            self2.spi.transfer_in_place(&mut bytes)?;
            self2.spi.transfer_in_place(values)?;
            Ok(())
        })
    }

    pub fn read_auto_inc(self: &mut Self, address: u8, values: &mut [u8]) -> Result<(), Error> {
        self.with_chip_selected(|self2| {
            let mut bytes = [Self::READ_nWRITE | Self::ADDRESS_AUTO_INCREMENT | (address & 0x3f)];
            self2.spi.transfer_in_place(&mut bytes)?;
            self2.spi.transfer_in_place(values)?;
            Ok(())
        })
    }
}

pub fn test_ims(ims: &mut IMS) -> Result<(), Error> {
    let who_am_i = ims.read(0x0f);
    if let Ok(who_am_i) = who_am_i {
        rprintln!("IMS, WHO_AM_I: {:x}", who_am_i);
        if who_am_i != 0x33 {
            rprintln!("IMS: ERROR: ID is not as expected");
            return Err(Error::WrongId)
        }
    } else {
        rprintln!("IMS, WHO_AM_I: error");
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

    if false {
        for _ in 0..4 {
            // temp[1] should change by ~1 for every 1 K change in temperatur
            // (and temp[0] will be 0 because of low-power mode)
            let mut temp = [0; 2];
            ims.read_auto_inc(0x0c, &mut temp)?;
            rprintln!("TEMP: {:02x}{:02x}", temp[1], temp[0]);

            let mut data = [0; 9];
            ims.read_auto_inc(0x27, &mut data)?;
            rprintln!("STATUS_REG: {:02x}", data[0]);
            rprintln!("OUT_X: {:02x}{:02x}", data[2], data[1]);
            rprintln!("OUT_Y: {:02x}{:02x}", data[4], data[3]);
            rprintln!("OUT_Z: {:02x}{:02x}", data[6], data[5]);
            rprintln!("FIFO REGS: {:02x}, {:02x}", data[7], data[8]);
        }
    }

    Ok(())
}

static FOR_INT: Mutex<RefCell<Option<(IMS, UpChannel)>>> = Mutex::new(RefCell::new(None));

#[used]
static ABC: u32 = 42;

fn write_ims_data_to_channel() {
    // Enter critical section
    cortex_m::interrupt::free(|cs| {
        // Obtain all Mutex protected resources
        if let &mut Some(ref mut for_int) = FOR_INT.borrow(cs).borrow_mut().deref_mut() {
            let (ims, channel) = for_int;

            //FIXME Do this with another interrupt or use RTIC!
            let mut data = [0; 9];
            let (data_a, data_b) = data.split_at_mut(2);
            let result1 = ims.read_auto_inc(0x0c, data_a);
            let result2 = ims.read_auto_inc(0x27, data_b);
            if result1.is_ok() && result2.is_ok() && data[2] != 0 {
                channel.write(&data);
            }

            // Clear event triggering the interrupt
            unsafe { pac::Peripherals::steal().EXTI.pr.write(|w| w.pif1().set_bit()); }
        }
    });
}

#[interrupt]
fn EXTI4_15() {
    write_ims_data_to_channel()
}

pub fn start_writing_to_rtt(ims: IMS<'static>, channel: UpChannel, syscfg: &mut pac::SYSCFG, exti: &mut pac::EXTI, nvic: &mut pac::NVIC) {
    syscfg.exticr3.modify(|_, w| w.exti9().pa9());
    exti.imr.modify(|_, w| w.mr9().set_bit());
    exti.rtsr.modify(|_, w| w.tr9().set_bit());

    // Enable EXTI IRQ, set prio 1 and clear any pending IRQs
    let irq = pac::Interrupt::EXTI4_15;
    unsafe {
        nvic.set_priority(irq, 1);
        cortex_m::peripheral::NVIC::unmask(irq);
    }
    cortex_m::peripheral::NVIC::unpend(irq);

    cortex_m::interrupt::free(move |cs| {
        *FOR_INT.borrow(cs).borrow_mut() = Some((ims, channel))
    });

    //FIXME remove
    loop {
        write_ims_data_to_channel()
    }
}

pub fn sleep(ims: &mut IMS) -> Result<(), Error> {
    let who_am_i = ims.read(0x0f);
    if let Ok(who_am_i) = who_am_i {
        rprintln!("IMS, WHO_AM_I: {:x}", who_am_i);
        if who_am_i != 0x33 {
            rprintln!("IMS: ERROR: ID is not as expected");
            return Err(Error::WrongId)
        }
    } else {
        rprintln!("IMS, WHO_AM_I: error");
        return Err(Error::WrongId)
    }

    ims.write_auto_inc(0x1e, &mut[
        // CTRL_REG0 (1e): disable pullup on DO pin because that should use less power
        0x90,
        // TEMP_CFG_REG (1f): disable temperature sensor
        0x00,
        // CTRL_REG1 (20): low-power, power-down
        0x08,
        // CTRL_REG2 (21): no high-pass filter
        0x00,
        // CTRL_REG3 (22): no interrupt
        0x00,
        // CTRL_REG4 (23): keep defaults
        0x00,
        // CTRL_REG5 (24): keep defaults
        0x00,
        // CTRL_REG6 (25): keep defaults
        0x00,
    ])?;

    // read REFERENCE (26) because datasheet suggests this when switching modes
    let _ = ims.read(0x26)?;

    if false {
        for _ in 0..4 {
            // temp[1] should change by ~1 for every 1 K change in temperatur
            // (and temp[0] will be 0 because of low-power mode)
            let mut temp = [0; 2];
            ims.read_auto_inc(0x0c, &mut temp)?;
            rprintln!("TEMP: {:02x}{:02x}", temp[1], temp[0]);

            let mut data = [0; 9];
            ims.read_auto_inc(0x27, &mut data)?;
            rprintln!("STATUS_REG: {:02x}", data[0]);
            rprintln!("OUT_X: {:02x}{:02x}", data[2], data[1]);
            rprintln!("OUT_Y: {:02x}{:02x}", data[4], data[3]);
            rprintln!("OUT_Z: {:02x}{:02x}", data[6], data[5]);
            rprintln!("FIFO REGS: {:02x}, {:02x}", data[7], data[8]);
        }
    }

    Ok(())
}
