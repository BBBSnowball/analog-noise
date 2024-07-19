use stm32f0xx_hal::pac::NVIC;
use stm32f0xx_hal::pac::RCC;
use stm32f0xx_hal::pac::STK;

unsafe fn rcc_set_defaults() {
    // values are reset values from reference manual, rev 10
    // -> actually, almost all are identical to reset() value
    (*RCC::PTR).cr.reset();
    (*RCC::PTR).cfgr.reset();
    (*RCC::PTR).cir.reset();
    (*RCC::PTR).apb2rstr.reset();
    (*RCC::PTR).apb1rstr.reset();
    (*RCC::PTR).ahbenr.reset();
    (*RCC::PTR).apb2enr.reset();
    (*RCC::PTR).apb1enr.reset();
    //(*RCC::PTR).bdcr.write(|w| w.reset().LSEDRV.high());
    (*RCC::PTR).bdcr.write(|w| w.bits(0x18));
    (*RCC::PTR).csr.reset();
    (*RCC::PTR).ahbrstr.reset();
    (*RCC::PTR).cfgr2.reset();
    (*RCC::PTR).cfgr3.reset();
    (*RCC::PTR).cr2.reset();
}

// see https://community.st.com/t5/stm32-mcus/how-to-jump-to-system-bootloader-from-application-code-on-stm32/ta-p/49424
// see https://github.com/embassy-rs/embassy/blob/main/embassy-boot-stm32/src/lib.rs
pub fn jump_to_bootloader() -> ! {
    unsafe {
        let bootloader_start = 0x1FFFC800; // for STM32F07x
        //let bootloaderVectorTable = std::mem::transmute::<_, const u32*>(bootloader_start);
        //let bootloaderStack = bootloaderVectorTable[0];
        //let bootloaderEntry = bootloaderVectorTable[1];

        let p = cortex_m::Peripherals::steal();

        cortex_m::interrupt::disable();

        /* Disable Systick timer */
        (*STK::ptr()).csr.write_with_zero(|w| w);

        rcc_set_defaults();

        /* Clear Interrupt Enable Register & Interrupt Pending Register */
        /*
        for i in 0..5*32 {
            NVIC::mask(i);
            NVIC::unpend(i);
        }
        */
        for i in 0..5 {
            (*NVIC::PTR).icer[i].write(0xffffffff);
            (*NVIC::PTR).icpr[i].write(0xffffffff);
        }

        cortex_m::interrupt::enable();

        p.SCB.vtor.write(bootloader_start as u32);
        cortex_m::asm::bootload(bootloader_start as *const u32);
    }
}
