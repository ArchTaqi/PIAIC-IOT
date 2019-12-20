#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let mut name = String::new();
    let mut subject_one = String::new();
    let mut subject_two = String::new();

    println!("enter your name");
    std::io::stdin().read_line(&mut name).ok().expect("Couldn't read line!");
    println!("enter marks gained in 1st subject: ");
    std::io::stdin().read_line(&mut subject_one).ok().expect("Couldn't read line!");
    println!("enter marks gained in 2nd subject: ");
    std::io::stdin().read_line(&mut subject_two).ok().expect("Couldn't read line!");

    let subject_one_marks = subject_one.trim().parse::<f64>().unwrap();
    let subject_two_marks = subject_two.trim().parse::<f64>().unwrap();

    let percentage = calculate_percentage(name,subject_one_marks,subject_two_marks);

    if percentage >= 70.0 {
        println!("Congrats!! You are Passed...");
    } else if percentage < 70.0 { 
        println!("You failed, try again...");
    }
}

fn calculate_percentage(name: String, subject_one_marks: f64, subject_two_marks: f64)->(f64){
    let percentage = ((subject_one_marks+subject_two_marks)/200.0)*100.0;
    println!("{0}! You'v got {1} % marks", name, percentage);
    percentage
}