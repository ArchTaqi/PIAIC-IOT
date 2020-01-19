extern crate notify_lib;
use notify_lib::notify::pop_up_notification;
mod lib;

pub mod notification {
    pub mod pop_up {
          pub fn info(text:String) {
             println!("Playing comedy movie {}",text);
          }
    }
    pub mod mail {
        pub fn info(text:String) {
              println!("Playing comedy movie {}",text);
        }
     }
 }

 fn main(){

    // Q1. Write a Rust Program,
    // ● Make a module with suitable name in main.rs
    // ● Make a sub module with an other name in first module
    // ● Define a simple function in sub module
    // ● Call that function from fn main
    notification::pop_up::info("Q1: UFC 246: McGregor vs. Cowboy Result...".to_string());
    notification::mail::info("Q1: UFC 246: McGregor vs. Cowboy Result...".to_string());

    // Q2. Write a Rust Program,
    // ● Make a library (lib.rs) alongwith main.rs
    // ● Make a module with suitable name in library
    // ● Make a sub module with an other name in first module
    // ● Define a simple function in sub module
    // ● use that library in main.rs
    // ● Call that function from fn main
    lib::notification::pop_up::info("Q2: UFC 246: McGregor vs. Cowboy Result...".to_string());
 

    // Q3. Write a Rust library,
    // ● Make a library package
    // ● Make a module with suitable name in library
    // ● Make a sub module with an other name in first module
    // ● Define a simple function in sub module
    // ● make a binary package
    // ● add your library in dependencies in cargo.toml
    // ● use your library in main.rs
    // ● Call that function from fn main
    pop_up_notification("Q3: UFC 246 Result.....".to_string());
 }