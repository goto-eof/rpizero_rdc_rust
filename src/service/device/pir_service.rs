use rust_gpiozero::DigitalInputDevice;
pub struct PirService {
    pir: DigitalInputDevice,
}

impl PirService {
    pub fn new(pin: u8) -> PirService {
        PirService {
            pir: DigitalInputDevice::new(pin),
        }
    }

    pub fn wait_for_motion(&mut self) -> () {
        let piir = &mut self.pir;
        piir.wait_for_active(None);
    }

    pub fn is_active(&mut self) -> bool {
        let piir = &mut self.pir;
        return piir.is_active();
    }

    pub fn wait_for_no_motion(&mut self) -> () {
        let piir = &mut self.pir;
        piir.wait_for_inactive(None);
    }
}
