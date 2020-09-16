use crate::errors::{SimulationError, SimulationResult};
use rppal::gpio::Gpio;
use rs_ws281x::ChannelBuilder;
use rs_ws281x::Controller;
use rs_ws281x::ControllerBuilder;
use rs_ws281x::StripType;

const LEDS_IN_LINE: i32 = 144;

pub fn build_controller(channel: usize, pin: i32) -> SimulationResult<Controller> {
    match ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            channel,
            ChannelBuilder::new()
                .pin(pin)
                .count(3 * LEDS_IN_LINE)
                .strip_type(StripType::Ws2811Rgb)
                .brightness(255)
                .build(),
        )
        .build()
    {
        Ok(controller) => Ok(controller),
        Err(_) => Err(SimulationError::LED),
    }
}

pub fn render_yin(line_num: i32, controller: &mut Controller) {
    let leds = controller.leds_mut(0);

    let part = LEDS_IN_LINE / 3;
    let position = LEDS_IN_LINE * line_num - 1;
    for num in position..position + LEDS_IN_LINE {
        if num > position + part && num < position + part * 2 {
            leds[num as usize] = [0, 0, 0, 0];
        }
        leds[num as usize] = [255, 255, 255, 0];
    }

    match controller.render() {
        Ok(_) => println!("yin"),
        Err(e) => println!("{:?}", e),
    };
}

pub fn render_yang(line_num: i32, controller: &mut Controller) {
    let leds = controller.leds_mut(0);

    let position = LEDS_IN_LINE * line_num - 1;
    for num in position..position + LEDS_IN_LINE {
        leds[num as usize] = [255, 255, 255, 0];
    }

    match controller.render() {
        Ok(_) => println!("yang"),
        Err(e) => println!("{:?}", e),
    };
}

pub fn pin_on(pin: u8) {
    if let Ok(gpio) = Gpio::new() {
        if let Ok(pin) = gpio.get(pin) {
            let mut pin = pin.into_output();
            pin.set_high();
        }
    }
}

pub fn pin_off(pin: u8) {
    if let Ok(gpio) = Gpio::new() {
        if let Ok(pin) = gpio.get(pin) {
            let mut pin = pin.into_output();
            pin.set_low();
        }
    }
}

// pub fn heaven_on(_colour: String, pin: u8) {
//     println!("----> heaven on, pin {}", pin);
//
//     on(pin);
// }
//
// pub fn heaven_off(pin: u8) {
//     println!("----> heaven off, pin {}", pin);
//
//     off(pin);
// }
//
// pub fn cloud_on(_colour: String, pin: u8) {
//     println!("----> cloud on, pin {}", pin);
//
//     on(pin);
// }
//
// pub fn cloud_off(pin: u8) {
//     println!("----> cloud off, pin {}", pin);
//
//     off(pin);
// }
//
// pub fn sun_on(_colour: String, pin: u8) {
//     println!("----> sun on, pin {}", pin);
//
//     on(pin);
// }
//
// pub fn sun_off(pin: u8) {
//     println!("----> sun off, pin {}", pin);
//
//     off(pin);
// }
//
// pub fn wind_on(_colour: String, pin: u8) {
//     println!("----> wind on, pin {}", pin);
//
//     on(pin);
// }
//
// pub fn wind_off(pin: u8) {
//     println!("----> wind off, pin {}", pin);
//
//     off(pin);
// }
//
// pub fn thunder_on(_colour: String, sound: String) {
//     println!("----> thunder on");
//
//     println!("play {}", sound);
// }
//
// pub fn thunder_off(sound: String) {
//     println!("----> thunder off");
//
//     println!("stop play {}", sound);
// }
//
// pub fn water_on(_colour: String, pin: u8) {
//     println!("----> water on, pin {}", pin);
//
//     on(pin);
// }
//
// pub fn water_off(pin: u8) {
//     println!("----> water off, pin {}", pin);
//
//     off(pin);
// }
//
// pub fn mountain_on(_colour: String, sound: String) {
//     println!("----> mountain on");
//
//     println!("play {}", sound);
// }
//
// pub fn mountain_off(sound: String) {
//     println!("----> mountain off");
//
//     println!("stop play {}", sound);
// }
//
// pub fn earth_on(_colour: String, _pin: u8) {
//     println!("----> earth on");
//
//     // on_off(pin);
// }
//
// pub fn earth_off(_pin: u8) {
//     println!("----> earth off");
//
//     // on_off(pin);
// }
