use crate::errors::SimulationResult;
use crate::schema::bindings;
use diesel::prelude::*;
use std::env;

#[derive(Serialize, Queryable, Debug)]
pub struct Settings {
    pub id: i32,
    pub default_colour: String,
    pub resting_colour: String,
    pub heaven_pin: i32,
    pub heaven_colour: String,
    pub cloud_pin: i32,
    pub cloud_colour: String,
    pub sun_pin: i32,
    pub sun_colour: String,
    pub wind_pin: i32,
    pub wind_colour: String,
    pub thunder_colour: String,
    pub water_pin: i32,
    pub water_colour: String,
    pub mountain_pin: i32,
    pub mountain_colour: String,
    pub earth_colour: String,
    pub multiply: String,
    pub bias: String,
    pub threshold: String,
}

pub fn get_settings(connection: &SqliteConnection) -> QueryResult<Settings> {
    bindings::table.find(1).get_result(connection)
}

impl Settings {
    pub fn read() -> SimulationResult<Settings> {
        let db = env::var("DB").unwrap_or_else(|err| {
            println!("DB: {}", err);
            std::process::exit(1);
        });
        let connection = SqliteConnection::establish(&db)?;

        Ok(get_settings(&connection)?)
    }
}
