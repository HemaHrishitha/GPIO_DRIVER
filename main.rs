#![no_std]
#![no_main]

pub mod gpio_driver; 
pub mod stm32l552xx;
  
use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use stm32_hal2::{
    clocks::Clocks,
    pac,
};

//use crate::gpio_driver::{Gpio_Pin, Gpio_Port};

// use crate::gpio_driver::Gpio_Pin;
// use crate::gpio_driver::Gpio_Port;


use panic_probe as _;

#[entry]
fn main() -> ! {

    use gpio_driver::{PortStruct,PinStruct};
    gpio_driver::hal_gpio_write;
    use crate::gpio_driver::PortStruct::{GPIOA,GPIOB,GPIOC};
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = pac::Peripherals::take().unwrap();

    let clock_cfg = Clocks::default();
    clock_cfg.setup().unwrap();

    let mut delay = Delay::new(cp.SYST, clock_cfg.systick());

    let mut led1 = PinStruct::new(PortStruct::GPIOA, 9);
    let mut led2 = PinStruct::new(PortStruct::GPIOB, 7);
    let mut led3 = PinStruct::new(PortStruct::GPIOC, 7);

    loop {
        led1.hal_gpio_write(GPIOA,0,9);
        delay.delay_ms(500);
        led1.hal_gpio_write(GPIOA,1,9);
        delay.delay_ms(500);

        led2.hal_gpio_write(GPIOB,0,7);
        delay.delay_ms(500);
        led2.hal_gpio_write(GPIOB,1,7);
        delay.delay_ms(500);

        led3.hal_gpio_write(GPIOC,0,7);
        delay.delay_ms(500);
        led3.hal_gpio_write(GPIOC,1,7);
        delay.delay_ms(500);
    }
}

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}