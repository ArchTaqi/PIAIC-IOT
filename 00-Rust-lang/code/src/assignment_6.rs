#![allow(dead_code)]
#![allow(unused_variables)]

// # Q. Write a Rust Program

// - 1) define a custom datatype using Struct with your favorite name. Which contains 3 fields, and 1 should be String Type.
// - 2) In main function:
//   - A. Create an instance of the above defined struct
//   - B. Print complete instance
//   - C. Print instance by calling its fields.
//   - D. create another instance by using fields of first instance
// - 3) define user defined function, User defined function should receive some arguments and return an instance of your above defined struct.
// - 4) in main function call user defined function, Print instance returned by the user defined function.

// > Important: ​ Use meaningful names while declaring variables, structs and functions. Avoid 'a', 'b', 'x', 'y', etc..



#[derive(Debug)]
struct UserLocation {
    city:String,
    country:String
}
#[derive(Debug)]
struct Student {
    first_name:String,
    last_name:String,
    location:UserLocation
}


fn main() {
    let default_location:(&str,&str) = ("Islamabad","Pakistan");

    let student1 = Student {
        first_name:String::from("Muhammad"),
        last_name:String::from("Taqi"),
        location: UserLocation {city:String::from("Islamabad"), country:String::from("Pakistan")}
     };

     println!("{:?}", student1);

     let student2 = Student {
        first_name:String::from("Muhammad"),
        last_name:String::from("Turab"),
        location: UserLocation {city:student1.location.city, country:student1.location.country}
     };
     println!("{:?}", student2);

     let new_student = registration("Ali", "Salman", default_location);
     println!("{:?}", new_student);
}

fn registration(first_name: &str,last_name: &str,location: (&str,&str))->Student {
    let (city,country) = location;
    let new_student = Student {
        first_name:first_name.to_string(), 
        last_name:last_name.to_string(), 
        location: UserLocation {city:city.to_string(), country:country.to_string()} 
    };
    return new_student;
}