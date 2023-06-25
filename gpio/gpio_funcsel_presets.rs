#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio0Func(pub u8);
impl Gpio0Func {
	pub const SPI0_RX: Self = Self(0x1);
	pub const UART0_RX: Self = Self(0x2);
	pub const I2C0_SDA: Self = Self(0x3);
	pub const PWM0_A: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_OVCUR_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio1Func(pub u8);
impl Gpio1Func {
	pub const SPIO_CSN: Self = Self(0x1);
	pub const UART0_RX: Self = Self(0x2);
	pub const I2C0_SCL: Self = Self(0x3);
	pub const PWM0_B: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_VBUS_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio2Func(pub u8);
impl Gpio2Func {
	pub const SPIO_SCK: Self = Self(0x1);
	pub const UART0_CTS: Self = Self(0x2);
	pub const I2C0_SDA: Self = Self(0x3);
	pub const PWM1_A: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_VBUS_EN: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio3Func(pub u8);
impl Gpio3Func {
	pub const SPIO_TX: Self = Self(0x1);
	pub const UART0_RTS: Self = Self(0x2);
	pub const I2C1_SCL: Self = Self(0x3);
	pub const PWM1_B: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_OVCUR_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio4Func(pub u8);
impl Gpio4Func {
	pub const SPIO_RX: Self = Self(0x1);
	pub const UART1_TX: Self = Self(0x2);
	pub const I2C0_SDA: Self = Self(0x3);
	pub const PWM2_A: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_VBUS_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio5Func(pub u8);
impl Gpio5Func {
	pub const SPIO_CSN: Self = Self(0x1);
	pub const UART1_RX: Self = Self(0x2);
	pub const I2C0_SCL: Self = Self(0x3);
	pub const PWM2_B: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_VBUS_EN: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio6Func(pub u8);
impl Gpio6Func {
	pub const SPIO_SCK: Self = Self(0x1);
	pub const UART1_CTS: Self = Self(0x2);
	pub const I2C1_SDA: Self = Self(0x3);
	pub const PWM3_A: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_OVCUR_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio7Func(pub u8);
impl Gpio7Func {
	pub const SPIO_TX: Self = Self(0x1);
	pub const UART1_RTS: Self = Self(0x2);
	pub const I2C1_SCL: Self = Self(0x3);
	pub const PWM3_B: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_VBUS_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio8Func(pub u8);
impl Gpio8Func {
	pub const SPI1_RX: Self = Self(0x1);
	pub const UART1_TX: Self = Self(0x2);
	pub const I2C0_SDA: Self = Self(0x3);
	pub const PWM4_A: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_VBUS_EN: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio9Func(pub u8);
impl Gpio9Func {
	pub const SPI1_CSN: Self = Self(0x1);
	pub const UART1_RX: Self = Self(0x2);
	pub const I2C0_SCL: Self = Self(0x3);
	pub const PWM4_B: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_OVCUR_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio10Func(pub u8);
impl Gpio10Func {
	pub const SPI1_SCK: Self = Self(0x1);
	pub const UART1_CTS: Self = Self(0x2);
	pub const I2C1_SDA: Self = Self(0x3);
	pub const PWM5_A: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_VBUS_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio11Func(pub u8);
impl Gpio11Func {
	pub const SPI1_TX: Self = Self(0x1);
	pub const UART1_RTS: Self = Self(0x2);
	pub const I2C1_SCL: Self = Self(0x3);
	pub const PWM5_B: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_VBUS_EN: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio12Func(pub u8);
impl Gpio12Func {
	pub const SPI1_RX: Self = Self(0x1);
	pub const UART0_TX: Self = Self(0x2);
	pub const I2C0_SDA: Self = Self(0x3);
	pub const PWM6_A: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_OVCUR_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio13Func(pub u8);
impl Gpio13Func {
	pub const SPI1_CSN: Self = Self(0x1);
	pub const UART0_RX: Self = Self(0x2);
	pub const I2C0_SCL: Self = Self(0x3);
	pub const PWM6_B: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_VBUS_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio14Func(pub u8);
impl Gpio14Func {
	pub const SPI1_SCK: Self = Self(0x1);
	pub const UART0_CTS: Self = Self(0x2);
	pub const I2C1_SDA: Self = Self(0x3);
	pub const PWM7_A: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_VBUS_EN: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio15Func(pub u8);
impl Gpio15Func {
	pub const SPI1_TX: Self = Self(0x1);
	pub const UART0_RTS: Self = Self(0x2);
	pub const I2C1_SCL: Self = Self(0x3);
	pub const PWM7_B: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_OVCUR_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio16Func(pub u8);
impl Gpio16Func {
	pub const SPIO_RX: Self = Self(0x1);
	pub const UART0_TX: Self = Self(0x2);
	pub const I2C0_SDA: Self = Self(0x3);
	pub const PWM0_A: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_VBUS_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio17Func(pub u8);
impl Gpio17Func {
	pub const SPIO_CSN: Self = Self(0x1);
	pub const UART0_RX: Self = Self(0x2);
	pub const I2C0_SCL: Self = Self(0x3);
	pub const PWM0_B: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_VBUS_EN: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio18Func(pub u8);
impl Gpio18Func {
	pub const SPIO_SCK: Self = Self(0x1);
	pub const UART0_CTS: Self = Self(0x2);
	pub const I2C1_SDA: Self = Self(0x3);
	pub const PWM1_A: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_OVCUR_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio19Func(pub u8);
impl Gpio19Func {
	pub const SPIO_TX: Self = Self(0x1);
	pub const UART0_RTS: Self = Self(0x2);
	pub const I2C1_SCL: Self = Self(0x3);
	pub const PWM1_B: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_VBUS_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio20Func(pub u8);
impl Gpio20Func {
	pub const SPIO_RX: Self = Self(0x1);
	pub const UART1_TX: Self = Self(0x2);
	pub const I2C0_SDA: Self = Self(0x3);
	pub const PWM2_A: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const CLOCK_GPIN0: Self = Self(0x8);
	pub const USB_VBUS_EN: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio21Func(pub u8);
impl Gpio21Func {
	pub const SPIO_CSN: Self = Self(0x1);
	pub const UART1_RX: Self = Self(0x2);
	pub const I2C0_SCL: Self = Self(0x3);
	pub const PWM2_B: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const CLOCK_GPOUT0: Self = Self(0x8);
	pub const USB_OVCUR_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio22Func(pub u8);
impl Gpio22Func {
	pub const SPIO_SCK: Self = Self(0x1);
	pub const UART1_CTS: Self = Self(0x2);
	pub const I2C1_SDA: Self = Self(0x3);
	pub const PWM3_A: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const CLOCK_GPIN1: Self = Self(0x8);
	pub const USB_VBUS_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio23Func(pub u8);
impl Gpio23Func {
	pub const SPIO_TX: Self = Self(0x1);
	pub const UART1_RTS: Self = Self(0x2);
	pub const I2C1_SCL: Self = Self(0x3);
	pub const PWM3_B: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const CLOCK_GPOUT1: Self = Self(0x8);
	pub const USB_VBUS_EN: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio24Func(pub u8);
impl Gpio24Func {
	pub const SPI1_RX: Self = Self(0x1);
	pub const UART1_TX: Self = Self(0x2);
	pub const I2C0_SDA: Self = Self(0x3);
	pub const PWM4_A: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const CLOCK_GPOUT2: Self = Self(0x8);
	pub const USB_OVCUR_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio25Func(pub u8);
impl Gpio25Func {
	pub const SPI1_CSN: Self = Self(0x1);
	pub const UART1_RX: Self = Self(0x2);
	pub const I2C0_SCL: Self = Self(0x3);
	pub const PWM4_B: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const CLOCK_GPOUT3: Self = Self(0x8);
	pub const USB_VBUS_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio26Func(pub u8);
impl Gpio26Func {
	pub const SPI1_SCK: Self = Self(0x1);
	pub const UART1_CTS: Self = Self(0x2);
	pub const I2C1_SDA: Self = Self(0x3);
	pub const PWM5_A: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_VBUS_EN: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio27Func(pub u8);
impl Gpio27Func {
	pub const SPI1_TX: Self = Self(0x1);
	pub const UART1_RTS: Self = Self(0x2);
	pub const I2C1_SCL: Self = Self(0x3);
	pub const PWM5_B: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_OVCUR_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio28Func(pub u8);
impl Gpio28Func {
	pub const SPI1_RX: Self = Self(0x1);
	pub const UART0_TX: Self = Self(0x2);
	pub const I2C0_SDA: Self = Self(0x3);
	pub const PWM6_A: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_VBUS_DET: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio29Func(pub u8);
impl Gpio29Func {
	pub const SPI1_CSN: Self = Self(0x1);
	pub const UART0_RX: Self = Self(0x2);
	pub const I2C0_SCL: Self = Self(0x3);
	pub const PWM6_B: Self = Self(0x4);
	pub const SIO: Self = Self(0x5);
	pub const PIO0: Self = Self(0x6);
	pub const PIO1: Self = Self(0x7);
	pub const USB_VBUS_EN: Self = Self(0x9);
	pub const NULL: Self = Self(0x1f);
}
