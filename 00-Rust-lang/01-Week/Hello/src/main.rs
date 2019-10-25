fn main() {

    // let a = true;
    // let b: bool = true;

    // let (x, y) = (1, 2);

    // let mut z = 5;
    // z = 6;


// Q # 1. Write a Rust program to perform mathematical operations between two numbers.
// Declare two integer variables and assign some values to them. Then print the result of addition,
// subtraction, division, and multiplication in between these two variables.
    let mut num1 = 5;
    let mut num2 = 7;
    // Output
    println!("sum of x and y is: {}", num1 + num2);
    println!("difference of x and y is: {}", num1 - num2);
    println!("Multiply of x and y is: {}", num1 * num2);
    println!("division of x and y is: {}", num1 / num2);


// Q # 2. Write a Rust program and declare a tuple for data of a Fruit name, its weight and its
// price. Destructure the tuple in separate variables and print them on your screen/terminal.
    let tatermelon_tuple = ("Watermelon", 5, 100);
    println!(" Fruit Name is {}, it's weight is {} and it's price is {}", tatermelon_tuple.0, tatermelon_tuple.1, tatermelon_tuple.2);


// Q # 3. Write a Rust program by initializing an array of cricket team’s names, and another array
// with their year of winning the world cup. Print the data as below:
// Output be like:
// Cricket Team: Pakistan - Year: 1992
// Cricket Team: SriLanka - Year: 1996
// (Note: For the sake of simplicity, limit only 5 teams’ names.)

    let teams = [1975, 1979, 1983,1987,1992,1996,1999,2003,2007,2011,2015,2019];
    let wins = ["West Indies","West Indies","India","Australia","Pakistan","Sri Lanka","Australia","Australia","Australia","India","Australia","England"];
    for (team, win) in teams.iter().zip(wins.iter()) {
        println!("Cricket Team: {} - Year: {}", team, win);
    }
}
