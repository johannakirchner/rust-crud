use self::models::Appointment;
use apoitment::*;
use appointments::*;
use diesel::prelude::*;
use schema::appointments;
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

    let post = diesel::update(appointments.find(other_id))
        .set(isapproved.eq(true))
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
    let mut quit = false;

    while !quit {
        println!("\nSelect the operation");
        println!("1. Create Appointment");
        println!("2. Delete Appointment");
        println!("3. Approve Appointment");
        println!("4. Show Appoitments");
        println!("5. Quit");

        let mut d = String::new();

        stdin().read_line(&mut d).unwrap();
        let d = d.trim_end();

        if d == "1" {
            new_appointment();
        }
        if d == "2" {
            remove_appointment();
        }
        if d == "3" {
            approve_appointment();
        }
        if d == "4" {
            show_appointments();
        }
        if d == "5" {
            quit = true;
        }
    }
}
