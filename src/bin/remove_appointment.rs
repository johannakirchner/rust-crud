use diesel::prelude::*;
use apoitment::*;
use std::env::args;

fn main() {
    use self::schema::appointments::dsl::*;

    
    let i = args()
        .nth(1)
        .expect("remove_appointment requires an id")
        .parse::<i32>()
        .expect("Invalid ID");

    let c = &mut connection();

    let num_deleted = diesel::delete(appointments.find(i))
        .execute(c)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}