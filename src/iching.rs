use crate::errors::SimulationResult;
use crate::settings::Settings;
use rand::distributions::{Distribution, Uniform};
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

        // line.on(line_num);

        line
    }

    // pub fn on(&self, line_num: u8) {
    //     match self {
    //         Line::Yin => yin(line_num),
    //         Line::Yang => yang(line_num),
    //     }
    // }

    // pub fn from_string(line: &String) -> Line {
    //     if *line == "Yin".to_string() {
    //         Line::Yin
    //     } else {
    //         Line::Yang
    //     }
    // }
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
    pub fn react(&self, settings: &Settings) -> SimulationResult<()> {
        Ok(())
    }

    // pub fn react(&self, connection: &SqliteConnection) -> IOracleResult<()> {
    //     let bindings = Binding::get(&connection)?;
    //     match self {
    //         Trigram {
    //             top: Line::Yang,
    //             middle: Line::Yang,
    //             bottom: Line::Yang,
    //         } => heaven_on(bindings.heaven_colour, bindings.heaven_pin as u8),
    //         Trigram {
    //             top: Line::Yin,
    //             middle: Line::Yang,
    //             bottom: Line::Yang,
    //         } => cloud_on(bindings.cloud_colour, bindings.cloud_pin as u8),
    //         Trigram {
    //             top: Line::Yang,
    //             middle: Line::Yin,
    //             bottom: Line::Yang,
    //         } => sun_on(bindings.sun_colour, bindings.sun_pin as u8),
    //         Trigram {
    //             top: Line::Yin,
    //             middle: Line::Yin,
    //             bottom: Line::Yang,
    //         } => wind_on(bindings.wind_colour, bindings.wind_pin as u8),
    //         Trigram {
    //             top: Line::Yang,
    //             middle: Line::Yang,
    //             bottom: Line::Yin,
    //         } => thunder_on(bindings.thunder_colour, bindings.thunder_sound),
    //         Trigram {
    //             top: Line::Yin,
    //             middle: Line::Yang,
    //             bottom: Line::Yin,
    //         } => water_on(bindings.wind_colour, bindings.water_pin as u8),
    //         Trigram {
    //             top: Line::Yang,
    //             middle: Line::Yin,
    //             bottom: Line::Yin,
    //         } => mountain_on(bindings.mountain_colour, bindings.mountain_sound),
    //         Trigram {
    //             top: Line::Yin,
    //             middle: Line::Yin,
    //             bottom: Line::Yin,
    //         } => earth_on(bindings.earth_colour, bindings.earth_pin as u8),
    //     }
    //
    //     Ok(())
    // }
}

pub struct Hexagram {
    pub top: Trigram,
    pub bottom: Trigram,
}

impl fmt::Display for Hexagram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // match *self {
        //     Line::Yin => write!(f, "Yin, open line"),
        //     Line::Yang => write!(f, "Yang, solid line"),
        // }
        write!(f, "----")
    }
}

// impl Hexagram {
//     pub fn name(&self, _connection: &SqliteConnection) -> IOracleResult<String> {
//         Ok("?".to_string())
//     }
// }
