#![allow(dead_code)]
#![allow(unused_variables)]
// mod types_and_variables;
// mod control_flow;



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

impl Student {
    fn new (FirstName: &str,LastName: &str,Location: UserLocation) -> Student {
        Student { FirstName: FirstName.to_string(),course: course.to_string(),marks: marks }
    },

    fn print(&self)->u32 { self.width * self.height }
}
 


fn main() {
    // types_and_variables::main();
    // control_flow::main();
    // check_positive_negative_zero(55);
    // print_arguments(25, 5.4, false);
    // println!("{:#?}", square_the_number(4));

    let default_location:(&str,&str) = ("Islamabad","Pakistan");


    let student1 = Student {
        first_name:String::from("Muhammad"),
        last_name:String::from("Taqi"),
        location: UserLocation {city:String::from("Islamabad"), country:String::from("Pakistan")}
     };

     println!("{:?}", student1);

     // create another instance by using fields of first instance

     let student2 = Student {
        first_name:String::from("Muhammad"),
        last_name:String::from("Turab"),
        location: UserLocation {city:student1.location.city, country:student1.location.country}
     };
     println!("{:?}", student2);

     let new_student = registration("Ali", "Salman", default_location);
     println!("{:?}", new_student);
    // let name = "Taqi".to_string();
    // let course = "IOT".to_string();


    // let student = Student::new(name.as_ref(), course.as_ref(),85);
    // println!("{:#?}",student.name);

    // let student_one:(&str,&str,u32) = (name.as_ref(), course.as_ref(), 28);
    // check_tuple(student_one);
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




fn check_tuple(student: (&str,&str,u32)) {
    println!("{:?}",student);
}




// Q # 1. Write a rust program, define a function that receives one argument of any suitable data
//        type and print whether the number is positive, negative or equal to zero. (hint: if/else)
fn check_positive_negative_zero(num: i32){
    if num > 0 {
        println!("number {} is positive", num);
    } else if num < 0 { 
        println!("number {} is positive", num);
    } else {
        println!("number {} is zero", num);
    }
}

// Q # 2. Write a rust program, define a function that receives 3 arguments of different data types
//        (integer, float, boolean) and print them on the console.
fn print_arguments(age: i32, height: f32, married: bool ){
    let maritial_status = if married {"married"} else {"single"};
    println!("Your Age is {}, height is {} and your are {}", age, height, maritial_status);
}

// Q # 3. Write a rust program, define a function that receives a number and return the number
//        itself and its square by using tuple.
fn square_the_number(number: i32)->(i32, i32){
    (number, number*number)
}


