use apoitment::*;
use std::io::{stdin};

fn main() {
    let c = &mut connection();

    let mut descrip = String::new();

    println!("Description of new appointment: ");
    stdin().read_line(&mut descrip).unwrap();
    let descrip = descrip.trim_end();

    let post = create_appointment(c, descrip);

    println!("Saved appointment with id {}", post.id);
}