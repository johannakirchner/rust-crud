use diesel::prelude::*;

#[derive(Queryable)]
pub struct Appointment {
    pub id: i32,
    pub descrip: String,
    pub isapproved: Option<bool>,
}

use crate::schema::appointments;

#[derive(Insertable)]
#[diesel(table_name = appointments)]
pub struct NewAppointment<'a> {
    pub descrip: &'a str
}