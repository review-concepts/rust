//Ownership

//*
// Each piece of data has one owning variable
// owner is responsible for cleaning up that data
// cleanup happens when the owner goes out of scope
// owner decided to mutibility
// non-primitive type rust moves ownership
// *//


// code 1
// fn main() {
//     let a= String::from("hello");
//     // let a =  --> is owner
//     let t = a; // dar inja dige moteghayere a ownere value hello nist, move peida karde be t
//     println!("i say, {}",a) // so we get this error: value borrowd here after move
// }


// use the mut to mutable owner

// code 2
// fn main() {
//     let mut a = String::from("hello");
//     a.push_str(" world");
//     println!("{}",a);
// }

// code 3
//
// fn say(s:String){
//     println!("{} world", s); // here s variable is owner of value
// }
//
// fn main() {
//     let a = String::from("hello");
//     say(a); // moved owner of value to say function
//     println!("{}",a); // we have not access to value anymore so cant use moved value
// }

// code 4

// fn function() -> String {
//     String::from("😻") // this returned as value
// }
//
// fn main() {
//     let a = function(); // a is owner of this value
//     println!("{}", function())
// }


// code 5
// clonning a String and transfer ownership of the clone

// fn say(s: String){
//     println!("{} world",s)
// }
// fn main() {
//     let a = String::from("hello");
//     say(a.clone()); // dar inja copie value a ro be say ferestad va banarin har kodom betore jodagone ownershipe data khodeshon hastand
//     println!("{}",a)
// }


// code 6
//seems  primitive type variable not following owenership rules. you research by self
// fn main() {
//     let a = 5;
//     let b = a;
//     println!("{}",a)
// }



// dar inja ma mafhome ownership va clone ra fahmidim
// har mafhomi ke sababe in shavad ma darim malekiate meghdario ja be ja ya move mikonim haeze ahamiat hast mesle entesab , funtion va ..