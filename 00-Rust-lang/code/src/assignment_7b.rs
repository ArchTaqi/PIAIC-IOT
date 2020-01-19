// lib.rs
pub mod notification {
    pub mod pop_up {
          pub fn info(text:String) {
             println!("Playing comedy movie {}",text);
          }
    }
 }

 // main.rs
 mod lib;

 fn main(){
    lib::notification::pop_up::info("UFC 246: McGregor vs. Cowboy Result...".to_string());
 }