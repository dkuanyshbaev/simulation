use crate::settings::Settings;
use crate::wires::*;
use rand::distributions::{Distribution, Uniform};
use rs_ws281x::Controller;
use std::fmt;

pub enum Line {
    Yin,  // open line
    Yang, // solid line
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Line::Yin => write!(f, "Yin, open line"),
            Line::Yang => write!(f, "Yang, solid line"),
        }
    }
}

impl Line {
    pub fn random() -> Line {
        let mut rng = rand::thread_rng();
        let line_range = Uniform::from(0..2);
        let line = if line_range.sample(&mut rng) == 0 {
            Line::Yin
        } else {
            Line::Yang
        };

        line
    }

    pub fn render(&self, line_num: i32, controller: &mut Controller, colour: &String) {
        match self {
            Line::Yin => render_yin(line_num, controller, colour),
            Line::Yang => render_yang(line_num, controller, colour),
        }
    }
}

pub struct Trigram {
    pub top: Line,
    pub middle: Line,
    pub bottom: Line,
}

impl fmt::Display for Trigram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Trigram {
                top: Line::Yang,
                middle: Line::Yang,
                bottom: Line::Yang,
            } => write!(f, "Heaven"),
            Trigram {
                top: Line::Yin,
                middle: Line::Yang,
                bottom: Line::Yang,
            } => write!(f, "Cloud"),
            Trigram {
                top: Line::Yang,
                middle: Line::Yin,
                bottom: Line::Yang,
            } => write!(f, "Sun"),
            Trigram {
                top: Line::Yin,
                middle: Line::Yin,
                bottom: Line::Yang,
            } => write!(f, "Wind"),
            Trigram {
                top: Line::Yang,
                middle: Line::Yang,
                bottom: Line::Yin,
            } => write!(f, "Thunder"),
            Trigram {
                top: Line::Yin,
                middle: Line::Yang,
                bottom: Line::Yin,
            } => write!(f, "Water"),
            Trigram {
                top: Line::Yang,
                middle: Line::Yin,
                bottom: Line::Yin,
            } => write!(f, "Mountain"),
            Trigram {
                top: Line::Yin,
                middle: Line::Yin,
                bottom: Line::Yin,
            } => write!(f, "Earth"),
        }
    }
}

impl Trigram {
    pub fn react(&self, settings: &Settings, controller: &mut Controller) {
        match self {
            Trigram {
                top: Line::Yang,
                middle: Line::Yang,
                bottom: Line::Yang,
            } => {
                heaven_on(settings.heaven_pin as u8);
                render_yang(1, controller, &settings.heaven_colour);
                render_yang(2, controller, &settings.heaven_colour);
                render_yang(3, controller, &settings.heaven_colour);
            }
            Trigram {
                top: Line::Yin,
                middle: Line::Yang,
                bottom: Line::Yang,
            } => {
                cloud_on(settings.cloud_pin as u8);
                render_yin(1, controller, &settings.cloud_colour);
                render_yang(2, controller, &settings.cloud_colour);
                render_yang(3, controller, &settings.cloud_colour);
            }
            Trigram {
                top: Line::Yang,
                middle: Line::Yin,
                bottom: Line::Yang,
            } => {
                sun_on(settings.sun_pin as u8);
                render_yang(1, controller, &settings.sun_colour);
                render_yin(2, controller, &settings.sun_colour);
                render_yang(3, controller, &settings.sun_colour);
            }
            Trigram {
                top: Line::Yin,
                middle: Line::Yin,
                bottom: Line::Yang,
            } => {
                wind_on(settings.wind_pin as u8);
                render_yin(1, controller, &settings.wind_colour);
                render_yin(2, controller, &settings.wind_colour);
                render_yang(3, controller, &settings.wind_colour);
            }
            Trigram {
                top: Line::Yang,
                middle: Line::Yang,
                bottom: Line::Yin,
            } => {
                thunder_on();
                render_yang(1, controller, &settings.thunder_colour);
                render_yang(2, controller, &settings.thunder_colour);
                render_yin(3, controller, &settings.thunder_colour);
            }
            Trigram {
                top: Line::Yin,
                middle: Line::Yang,
                bottom: Line::Yin,
            } => {
                water_on(settings.water_pin as u8);
                render_yin(1, controller, &settings.water_colour);
                render_yang(2, controller, &settings.water_colour);
                render_yin(3, controller, &settings.water_colour);
            }
            Trigram {
                top: Line::Yang,
                middle: Line::Yin,
                bottom: Line::Yin,
            } => {
                mountain_on(settings.mountain_pin as u8);
                render_yang(1, controller, &settings.mountain_colour);
                render_yin(2, controller, &settings.mountain_colour);
                render_yin(3, controller, &settings.mountain_colour);
            }
            Trigram {
                top: Line::Yin,
                middle: Line::Yin,
                bottom: Line::Yin,
            } => {
                earth_on();
                render_yin(1, controller, &settings.earth_colour);
                render_yin(2, controller, &settings.earth_colour);
                render_yin(3, controller, &settings.earth_colour);
            }
        }
    }
}

pub struct Hexagram {
    pub top: Trigram,
    pub bottom: Trigram,
}

impl fmt::Display for Hexagram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "----")
    }
}
