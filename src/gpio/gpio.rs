// src/gpio.rs

const GPIO_BASE: u32 = 0x40014000;
const NUM_GPIO: u8 = 30;

pub enum PinDirection {
	In,
	Out
}

pub enum PinValue {
	High,
	Low
}

pub struct GpioPin {
	pub control_register: *const u32,
	pub status_register: *const u32,
}

impl GpioPin {
	pub fn new(pin: u8) -> Result<Self, String> {

		// Catch invalid pin numbers
		if pin > NUM_GPIO - 1 {
			return Err("Invalid pin number".to_string());
		}

		// Create the GpioPin instance
		let gpio_pin = GpioPin {

			// Calculate the base addresses for the pin's control and status registers
			status_register: (GPIO_BASE + (pin as u32 * 0x8)) as *const u32,
			control_register: (GPIO_BASE + (pin as u32 * 0x8) + 0x4) as *const u32,
		};

		Ok(gpio_pin)
	}

	pub fn set_direction(&self, pin: u8, direction: PinDirection) {
		// set the direction (input/output)
	}

	pub fn read_pin_value(&self, pin: u8) -> PinValue {
		// read the current value of the pin
		PinValue::High
	}

	pub fn write_pin_value(&self, pin: u8, value: PinValue) {
		// set the value of the pin
	}
}