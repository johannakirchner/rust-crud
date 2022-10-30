use self::models::*;
use diesel::prelude::*;
use apoitment::*;

fn main() {
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