use crate::{LedRequest, LedResponse};

use super::device::{led_service::LedService, pir_service::PirService};

pub struct RdcService {
    led_service: LedService,
    pir_service: PirService,
}

const LED_PIN: u8 = 4;
const MOTION_SENSOR_PIN: u8 = 17;

impl RdcService {
    pub fn new() -> RdcService {
        RdcService {
            led_service: LedService::new(LED_PIN),
            pir_service: PirService::new(MOTION_SENSOR_PIN),
        }
    }

    pub fn is_pir_active(&mut self) -> bool {
        return self.pir_service.is_active();
    }

    pub fn wait_for_motion(&mut self) -> () {
        self.pir_service.wait_for_motion();
    }

    pub fn wait_for_no_motion(&mut self) -> () {
        self.pir_service.wait_for_no_motion();
    }

    pub fn led_on_or_off(&self, led_request: LedRequest) -> LedResponse {
        if led_request.on {
            return self.led_on();
        }
        return self.led_off();
    }

    fn led_on(&self) -> LedResponse {
        if !self.led_service.is_on() {
            self.led_service.on();
        }
        return LedResponse {
            status: self.led_service.is_on(),
        };
    }

    fn led_off(&self) -> LedResponse {
        if self.led_service.is_on() {
            self.led_service.off();
        }
        return LedResponse {
            status: self.led_service.is_on(),
        };
    }

    pub fn is_led_on(&self) -> LedResponse {
        return LedResponse {
            status: self.led_service.is_on(),
        };
    }
}
