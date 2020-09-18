use crate::errors::{SimulationError, SimulationResult};
use crate::settings::Settings;
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

pub fn render_yin(line_num: i32, controller: &mut Controller, colour: &String) {
    let leds = controller.leds_mut(0);
    let (a, b, c) = parse_colour(colour);

    let part = LEDS_IN_LINE / 3;
    let position = LEDS_IN_LINE * line_num - 1;
    for num in position..position + LEDS_IN_LINE {
        if num > position + part && num < position + part * 2 {
            leds[num as usize] = [0, 0, 0, 0];
        }
        leds[num as usize] = [a, b, c, 0];
    }

    match controller.render() {
        Ok(_) => println!("yin"),
        Err(e) => println!("{:?}", e),
    };
}

pub fn render_yang(line_num: i32, controller: &mut Controller, colour: &String) {
    let leds = controller.leds_mut(0);
    let (a, b, c) = parse_colour(colour);

    let position = LEDS_IN_LINE * line_num - 1;
    for num in position..position + LEDS_IN_LINE {
        leds[num as usize] = [a, b, c, 0];
    }

    match controller.render() {
        Ok(_) => println!("yang"),
        Err(e) => println!("{:?}", e),
    };
}

pub fn pin_on(pin: u8) {
    println!("--------> pin {}: on", pin);
    if let Ok(gpio) = Gpio::new() {
        if let Ok(pin) = gpio.get(pin) {
            let mut pin = pin.into_output();
            pin.set_high();
        }
    }
}

pub fn pin_off(pin: u8) {
    println!("--------> pin {}: off", pin);
    if let Ok(gpio) = Gpio::new() {
        if let Ok(pin) = gpio.get(pin) {
            let mut pin = pin.into_output();
            pin.set_low();
        }
    }
}

pub fn heaven_on(_colour: &String, pin: u8) {
    pin_on(pin);
}

pub fn heaven_off(pin: u8) {
    pin_off(pin);
}

pub fn cloud_on(_colour: &String, pin: u8) {
    pin_on(pin);
}

pub fn cloud_off(pin: u8) {
    pin_off(pin);
}

pub fn sun_on(_colour: &String, pin: u8) {
    pin_on(pin);
}

pub fn sun_off(pin: u8) {
    pin_off(pin);
}

pub fn wind_on(_colour: &String, pin: u8) {
    pin_on(pin);
}

pub fn wind_off(pin: u8) {
    pin_off(pin);
}

pub fn thunder_on(_colour: &String) {
    println!("--------> play thunder sound");
}

pub fn thunder_off(_sound: &String) {
    println!("--------> stop thunder sound");
}

pub fn water_on(_colour: &String, pin: u8) {
    pin_on(pin);
}

pub fn water_off(pin: u8) {
    pin_off(pin);
}

pub fn mountain_on(_colour: &String, _pin: u8) {
    println!("--------> play mountain sound");
}

pub fn mountain_off(_sound: &String) {
    println!("--------> stop mountain sound");
}

pub fn earth_on(_colour: &String) {
    println!("--------> earth on");
}

// pub fn earth_off(_pin: u8) {
//     println!("--------> earth off");
// }

pub fn rest(_settings: Settings) {
    println!("--------> all off");
}

fn parse_colour(colour: &String) -> (u8, u8, u8) {
    let mut str_buff = colour.clone();
    let mut rgb = (255, 255, 255);

    // colour string format:  "rgb(108, 73, 211)"
    let mut str_buff: String = str_buff.drain(4..).collect();
    str_buff.pop();
    let mut str_parts = str_buff.split(", ");
    let parts: Vec<&str> = str_parts.collect();

    if let Ok(part) = parts[0].parse::<u8>() {
        rgb.0 = part;
    }
    if let Ok(part) = parts[1].parse::<u8>() {
        rgb.1 = part;
    }
    if let Ok(part) = parts[2].parse::<u8>() {
        rgb.2 = part;
    }

    rgb
}
