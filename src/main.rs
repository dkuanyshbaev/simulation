mod errors;
mod iching;
mod settings;
mod wires;

use errors::SimulationResult;
use iching::{Hexagram, Line, Trigram};
use settings::Settings;
use std::time::Duration;
use std::{process, thread};

fn main() {
    println!("iOracle simulation");
    println!("------------------");

    let settings = Settings::read().unwrap_or_else(|_| {
        println!("Can't read settings from db!");
        process::exit(1);
    });

    if let Err(error) = run(settings) {
        println!("Error: {:?}!", error);
    }
}

fn run(settings: Settings) -> SimulationResult<()> {
    let line1 = Line::random();
    println!("Line 1: {}", line1);
    thread::sleep(Duration::from_secs(1));

    let line2 = Line::random();
    println!("Line 2: {}", line2);
    thread::sleep(Duration::from_secs(1));

    let line3 = Line::random();
    println!("Line 3: {}", line3);
    thread::sleep(Duration::from_secs(1));

    let top_trigram = Trigram {
        top: line1,
        middle: line2,
        bottom: line3,
    };
    println!("Top Trigram: {}", top_trigram);
    top_trigram.react(&settings)?;
    thread::sleep(Duration::from_secs(1));

    let line4 = Line::random();
    println!("Line 4: {}", line4);
    thread::sleep(Duration::from_secs(1));

    let line5 = Line::random();
    println!("Line 5: {}", line5);
    thread::sleep(Duration::from_secs(1));

    let line6 = Line::random();
    println!("Line 6: {}", line6);
    thread::sleep(Duration::from_secs(1));

    let bottom_trigram = Trigram {
        top: line4,
        middle: line5,
        bottom: line6,
    };
    println!("Bottom Trigram: {}", bottom_trigram);
    bottom_trigram.react(&settings)?;
    thread::sleep(Duration::from_secs(1));

    let hexagram = Hexagram {
        top: top_trigram,
        bottom: bottom_trigram,
    };
    println!("Hexagram: {}", hexagram);

    Ok(())
}
