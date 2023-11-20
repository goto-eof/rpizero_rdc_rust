use rust_gpiozero::LED;

pub struct LedService {
    led: LED,
}

impl LedService {
    pub fn new(pin: u8) -> LedService {
        let led: LED = LED::new(pin);
        LedService { led: led }
    }

    pub fn on(&self) -> () {
        if !self.led.is_lit() {
            self.led.on();
        }
    }

    pub fn off(&self) -> () {
        if self.led.is_lit() {
            self.led.off();
        }
    }

    pub fn is_on(&self) -> bool {
        return self.led.is_lit();
    }
}
