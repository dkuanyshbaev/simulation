// use crate::errors::{SimulationError, SimulationResult};
use crate::errors::SimulationResult;

pub struct Settings {
    pub file_name: String,
    pub heaven_pin: i32,
    pub heaven_colour: String,
    pub cloud_pin: i32,
    pub cloud_colour: String,
    pub sun_pin: i32,
    pub sun_colour: String,
    pub wind_pin: i32,
    pub wind_colour: String,
    pub thunder_sound: String,
    pub thunder_colour: String,
    pub water_pin: i32,
    pub water_colour: String,
    pub mountain_sound: String,
    pub mountain_colour: String,
    pub earth_pin: i32,
    pub earth_colour: String,
    pub multiply: String,
    pub bias: String,
    pub threshold: String,
}

impl Settings {
    pub fn read() -> SimulationResult<Settings> {
        let settings = Settings {
            file_name: "file".to_string(),
            heaven_colour: "green".to_string(),
            heaven_pin: 0,
            cloud_colour: "green".to_string(),
            cloud_pin: 0,
            sun_colour: "green".to_string(),
            sun_pin: 0,
            wind_colour: "green".to_string(),
            wind_pin: 0,
            thunder_colour: "green".to_string(),
            thunder_sound: "green".to_string(),
            water_colour: "green".to_string(),
            water_pin: 0,
            mountain_colour: "green".to_string(),
            mountain_sound: "green".to_string(),
            earth_colour: "green".to_string(),
            earth_pin: 0,
            multiply: "green".to_string(),
            bias: "green".to_string(),
            threshold: "green".to_string(),
        };

        // Err(SimulationError::DB)
        Ok(settings)
    }
}

// use super::schema::bindings;
// use crate::errors::{IOracleError, IOracleResult};
// use rocket_contrib::databases::diesel::prelude::*;
// use rocket_contrib::databases::diesel::SqliteConnection;
// use std::fs::File;
// use std::path::Path;
//
// #[derive(Serialize, Queryable, Identifiable, Debug)]
// pub struct Binding {
//     pub id: i32,
//     pub file_name: String,
//     pub heaven_pin: i32,
//     pub heaven_colour: String,
//     pub cloud_pin: i32,
//     pub cloud_colour: String,
//     pub sun_pin: i32,
//     pub sun_colour: String,
//     pub wind_pin: i32,
//     pub wind_colour: String,
//     pub thunder_sound: String,
//     pub thunder_colour: String,
//     pub water_pin: i32,
//     pub water_colour: String,
//     pub mountain_sound: String,
//     pub mountain_colour: String,
//     pub earth_pin: i32,
//     pub earth_colour: String,
//     pub multiply: String,
//     pub bias: String,
//     pub threshold: String,
// }
//
// #[derive(Serialize, Deserialize, Insertable, FromForm, AsChangeset, Debug)]
// #[table_name = "bindings"]
// pub struct UpdatedBinding {
//     file_name: String,
//     heaven_pin: i32,
//     heaven_colour: String,
//     cloud_pin: i32,
//     cloud_colour: String,
//     sun_pin: i32,
//     sun_colour: String,
//     wind_pin: i32,
//     wind_colour: String,
//     thunder_sound: String,
//     thunder_colour: String,
//     water_pin: i32,
//     water_colour: String,
//     mountain_sound: String,
//     mountain_colour: String,
//     earth_pin: i32,
//     earth_colour: String,
//     multiply: String,
//     bias: String,
//     threshold: String,
// }
//
// impl Binding {
//     pub fn get(connection: &SqliteConnection) -> QueryResult<Binding> {
//         bindings::table.find(1).get_result(connection)
//     }
//
//     pub fn update(connection: &SqliteConnection, bindings: UpdatedBinding) -> QueryResult<usize> {
//         diesel::update(bindings::table.find(1))
//             .set(bindings)
//             .execute(connection)
//     }
//
//     pub fn write_to_file(connection: &SqliteConnection) -> IOracleResult<()> {
//         let current_bindings = Self::get(connection)?;
//         let path = Path::new(&current_bindings.file_name);
//
//         match File::create(&path) {
//             Err(_) => Err(IOracleError::InternalServerError),
//             Ok(file) => match serde_json::to_writer(file, &current_bindings) {
//                 Err(_) => Err(IOracleError::InternalServerError),
//                 Ok(_) => Ok(()),
//             },
//         }
//     }
//
//     // pub fn read_from_file(file_name: String) -> IOracleResult<UpdatedBinding> {
//     //     let path = Path::new(&file_name);
//     //
//     //     match File::open(&path) {
//     //         Err(_) => Err(IOracleError::InternalServerError),
//     //         Ok(file) => match serde_json::from_reader(file) {
//     //             Err(_) => Err(IOracleError::InternalServerError),
//     //             Ok(bindings) => Ok(bindings),
//     //         },
//     //     }
//     // }
// }
