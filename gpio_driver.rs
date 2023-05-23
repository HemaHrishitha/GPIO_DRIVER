const GPIO_MODE_POS: u32 = 0;
const GPIO_MODE: u32 = 0x3u32 << GPIO_MODE_POS;
const MODE_INPUT: u32 = 0x0u32 << GPIO_MODE_POS;
const MODE_OUTPUT: u32 = 0x1u32 << GPIO_MODE_POS;

// const GPIO_OSPEEDR_OSPEED0_POS: u32 = 0;
// const GPIO_OSPEEDR_OSPEED0_MSK: u32 = 0x3 << GPIO_OSPEEDR_OSPEED0_POS;
// const GPIO_OSPEEDR_OSPEED0: u32 = GPIO_OSPEEDR_OSPEED0_MSK;

const GPIO_Reset : u32 = 0;
const GPIO_Set : u32=1;

const GPIO_NUMBER : u32 = 0;

pub struct PinStruct {
    /// The GPIO Port letter. Eg A, B, C.
    pub port_para: PortStruct,
    /// The pin number: 1 - 15.
    pub pin_para: u8,
    pub fn hal_gpio_write(),
}

impl PinStruct {
    pub fn new(port_para: PortStruct, pin_para: u8) -> Self {
        Self { port_para, pin_para}
    }
}

pub enum PortStruct {
    GPIOA,
    GPIOB,
    GPIOC,
    GPIOD,
    GPIOE,
    GPIOF,
    GPIOG,
    GPIOH,
}

pub struct GPIOx{
    pub MODER : u32,  //GPIO port mode register
    pub OTYPER : u32,  //GPIO port output type register
    pub OSPEEDR : u32,  //GPIO port output speed register
    pub ODR : u32,  //GPIO port output data register
    pub PUPDR : u32,  // GPIO port pull-up/pull-down register
    pub IDR : u32,  //GPIO port input data register
    pub RESERVED : u32,  //Reserved
    pub BRR : u32,  //GPIO Bit Reset register
    pub BSRR : u32,  //GPIO port bit set/reset  register
}

pub struct GPIOInit{
    pub pin: u32,  //Specifies the GPIO pins to be configured
    pub mode: u32,  //Specifies the operating mode for the selected pins
    pub pull: u32,  // Specifies the Pull-up or Pull-Down activation for the selected pins
    pub speed : u32,  //Specifies the speed for the selected pins
    pub alternate : u32, //Peripheral to be connected to the selected pins
}

pub fn hal_gpio_init(GPIO_Port : &mut GPIOx , GPIO_Pin : &mut GPIOInit){

    let mut position: u32 = 0;
    let mut iocurrent :u32;
    let mut temp: u32;

while(GPIO_Pin.pin>> position) != 0 {

    iocurrent = (GPIO_Pin. pin) & ( 1 << position);

    if iocurrent != 0 {

       if ( GPIO_Pin. mode & GPIO_MODE) == MODE_OUTPUT{

// temp = GPIO_Port.OSPEEDR;
// temp &= !(GPIO_OSPEEDR_OSPEED0 << (position * 2));
// temp |= GPIO_Pin.speed << (position * 2);
// GPIO_Port.OSPEEDR = temp;

temp = GPIO_Port.OTYPER;
temp &= !(1 << position);
temp |= ((GPIO_Pin.mode & MODE_OUTPUT) >> GPIO_MODE_POS) << position;
GPIO_Port.OTYPER = temp;

temp = GPIO_Port.PUPDR;
temp &= !(3 << (position * 2));
temp |= GPIO_Pin.pull << (position * 2);
GPIO_Port.PUPDR = temp;

}
}

}
}
// enum PinState {
//     Reset = 0,
//     Set = 1,
// }


pub fn hal_gpio_write(GPIO_Port: &mut GPIOx, GPIO_state:u32, GPIO_PortPin:u32, pin_para: PinStruct)
 { 

    if GPIO_state == GPIO_Set{
     GPIO_Port.BSRR = GPIO_PortPin;
    }
    else {
    GPIO_Port.BRR = GPIO_PortPin;
    }

}


pub fn hal_Gpio_Toggle(GPIO_Port: &mut GPIOx, GPIO_state:u32, GPIO_PortPin:u32)
{
    let mut odr: u32;
    odr = GPIO_Port.ODR;
    GPIO_Port.BSRR = ((odr & GPIO_PortPin) << GPIO_NUMBER) | (!odr & GPIO_PortPin);
}



// }

// pub fn set_high(&mut self) {
//     self.set_state(PinState::High);
// }

// pub fn set_low(&mut self) {
//     self.set_state(PinState::Low);
// }

// pub fn toggle(&mut self) {
//     if self.is_high() {
//         self.set_low();
//     } else {
//         self.set_high();
//     }
// }
