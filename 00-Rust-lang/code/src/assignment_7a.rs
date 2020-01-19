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
    notification::pop_up::info("UFC 246: McGregor vs. Cowboy Result...".to_string());
    notification::mail::info("UFC 246: McGregor vs. Cowboy Result...".to_string());
 }