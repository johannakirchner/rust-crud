use self::models::Appointment;
use diesel::prelude::*;
use apoitment::*;
use std::env::args;
use std::io::{stdin};

fn main() {
    use self::schema::appointments::dsl::{appointments, isapproved};

    let id = args()
        .nth(1)
        .expect("approve_appointment requires an id")
        .parse::<i32>()
        .expect("Invalid ID");
    let c = &mut connection();

    println!("Approve appointment with id {}?", id);


    let mut approve = String::new();
    stdin().read_line(&mut approve).unwrap();

    let approve: bool = match approve.trim().parse() {
        Ok(num) => num,
        Err(_) => false,
    };

    let post = diesel::update(appointments.find(id))
        .set(isapproved.eq(approve))
        .get_result::<Appointment>(c)
        .unwrap();
    println!("Altered appointment {}", post.id);
}