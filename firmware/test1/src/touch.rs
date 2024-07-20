// TOUCH1, PA6, TSC_G2_IO3
// TOUCH2, PA7, TSC_G2_IO4
// TOUCH3, PA2, TSC_G1_IO3

use crate::hal::{
    gpio::*,
    gpio::gpioa::*,
    pac,
    pac::interrupt,
    prelude::*,
    rcc::Rcc,
    tsc,
};
use core::{cell::RefCell, convert::Infallible, ops::DerefMut};
use cortex_m::interrupt::Mutex;
use rtt_target::{UpChannel, rprintln};

pub fn test_touch(tsc: pac::TSC, rcc: &mut Rcc, touch1: PA6<Input<Floating>>, touch2: PA7<Input<Floating>>, touch3: PA2<Input<Floating>>, pa0: PA0<Input<Floating>>) {
    let mut tsc = crate::hal::tsc::Tsc::tsc(tsc, rcc, None);
    let (mut touch1, mut touch2, mut touch3, mut pa0) = cortex_m::interrupt::free(move |cs| {
        (
            touch1.into_alternate_af3(cs),
            touch2.into_alternate_af3(cs),
            touch3.into_alternate_af3(cs),
            pa0.into_alternate_af3(cs),
        )
    });
    tsc.setup_sample_group(&mut touch2);
    tsc.setup_sample_group(&mut pa0);
    tsc.enable_channel(&mut touch1);
    tsc.enable_channel(&mut touch3);
    tsc.start();
    for _ in 0..4 {
        if let Ok(_) = tsc.acquire() {
            //rprintln!("TOUCH: {}, {}, {}", tsc.read(&mut touch1).unwrap(), tsc.read(&mut touch2).unwrap(), tsc.read(&mut touch3).unwrap());
            rprintln!("TOUCH: {}, {}", tsc.read(&mut touch1).unwrap(), tsc.read(&mut touch3).unwrap());
        }
    }
}
