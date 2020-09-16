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

    pub fn show(&self, line_num: i32, controller: &mut Controller) {
        match self {
            Line::Yin => render_yin(line_num, controller),
            Line::Yang => render_yang(line_num, controller),
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
    pub fn react(&self, settings: &Settings) {
        match self {
            Trigram {
                top: Line::Yang,
                middle: Line::Yang,
                bottom: Line::Yang,
            } => heaven_on(&settings.heaven_colour, settings.heaven_pin as u8),
            Trigram {
                top: Line::Yin,
                middle: Line::Yang,
                bottom: Line::Yang,
            } => cloud_on(&settings.cloud_colour, settings.cloud_pin as u8),
            Trigram {
                top: Line::Yang,
                middle: Line::Yin,
                bottom: Line::Yang,
            } => sun_on(&settings.sun_colour, settings.sun_pin as u8),
            Trigram {
                top: Line::Yin,
                middle: Line::Yin,
                bottom: Line::Yang,
            } => wind_on(&settings.wind_colour, settings.wind_pin as u8),
            Trigram {
                top: Line::Yang,
                middle: Line::Yang,
                bottom: Line::Yin,
            } => thunder_on(&settings.thunder_colour, &settings.thunder_sound),
            Trigram {
                top: Line::Yin,
                middle: Line::Yang,
                bottom: Line::Yin,
            } => water_on(&settings.wind_colour, settings.water_pin as u8),
            Trigram {
                top: Line::Yang,
                middle: Line::Yin,
                bottom: Line::Yin,
            } => mountain_on(&settings.mountain_colour, &settings.mountain_sound),
            Trigram {
                top: Line::Yin,
                middle: Line::Yin,
                bottom: Line::Yin,
            } => earth_on(&settings.earth_colour, settings.earth_pin as u8),
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
