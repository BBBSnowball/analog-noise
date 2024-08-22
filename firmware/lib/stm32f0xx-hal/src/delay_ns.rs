// This is in its own file because the import of DelayNs would make
// all delay_us calls ambiguous.
use crate::delay::Delay;
use embedded_hal_new::delay::DelayNs;

impl DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        self.delay_ticks(ns * self.scale / 1000)
    }
}
