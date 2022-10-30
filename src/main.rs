use self::models::Appointment;
use apoitment::*;
use appointments::*;
use diesel::prelude::*;
use schema::appointments;
use std::env::args;
use std::io::stdin;

pub mod models;
pub mod schema;

fn show_appointments() {
    use self::schema::appointments::dsl::*;

    let c = &mut connection();
    let result = appointments
        .load::<Appointment>(c)
        .expect("Error loading appointments");

    println!("Displaying {} appointments.\n", result.len());
    for a in result {
        println!("Id: {}", a.id);
        println!("Description: {}", a.descrip);
        println!("Is Approved: {:?}", a.isapproved);
    }
    println!("------")
}

fn approve_appointment() {
    use self::schema::appointments::dsl::appointments;

    println!("Id to approve: ");

    let mut d = String::new();
    stdin().read_line(&mut d).unwrap();
    let d = d.trim_end();

    let other_id = d.parse::<i32>().expect("Invalid ID");

    let c = &mut connection();

    println!("Approve appointment with id {}?", other_id);

    let mut approve = String::new();
    stdin().read_line(&mut approve).unwrap();

    let approve: bool = match approve.trim().parse() {
        Ok(num) => num,
        Err(_) => false,
    };

    let post = diesel::update(appointments.find(other_id))
        .set(isapproved.eq(approve))
        .get_result::<Appointment>(c)
        .unwrap();
    println!("Altered appointment {}", post.id);
}

fn new_appointment() {
    let c = &mut connection();

    let mut d = String::new();

    println!("Description of new appointment: ");
    stdin().read_line(&mut d).unwrap();
    let d = d.trim_end();

    let post = create_appointment(c, d);

    println!("Saved appointment with id {}", post.id);
}

fn remove_appointment() {
    use self::schema::appointments::dsl::*;

    println!("Id to remove: ");


    let mut d = String::new();
    stdin().read_line(&mut d).unwrap();
    let d = d.trim_end();

    let i = d.parse::<i32>().expect("Invalid ID");

    let c = &mut connection();

    let num_deleted = diesel::delete(appointments.find(i))
        .execute(c)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}

fn main() {
    show_appointments();
    new_appointment();
    show_appointments();
    remove_appointment();
    show_appointments();
    approve_appointment();
    show_appointments();
}
