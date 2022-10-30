use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::{NewAppointment, Appointment};

pub fn create_appointment(c: &mut PgConnection, descrip: &str) -> Appointment {
    use crate::schema::appointments;

    let new_app = NewAppointment { descrip };

    diesel::insert_into(appointments::table)
        .values(&new_app)
        .get_result(c)
        .expect("Error saving new appointment")
}