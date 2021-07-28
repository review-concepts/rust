/*
    enums or enumerations: enum is a way to express that a value can be one of a finit set of possible Values
    enums are one of the ways of defining custom type is rust, structs are the other, we cover that in next secttion

    -   when enums are usefull?
            lets say we are writing a program having to do with soccer, in soccer there are several position a player can play: Goalkeeper, Fullback,  Center, Defending and ...
             there are two aspects of this property of player that make this a good case to model with an enum
             -  can only be one value at a time:
                player can only be in one one of these position at a time, while players may change positions,
             -  can list(enumeration) all possible values
                we can list all the possibilites for the values of position in soccer
                there is finit number of possible values and dont need change
                our soccer program wont need to allow users to define new soccer positions


    -   syntax for defining an enum
            see the down blew code example
    -   syntax for using an enum
            see the down blew code example
    -   defining enum variants that hold data
            see the last down blew code example
    -   enums and match expression
            see the last down blew code example
 */








/*
    syntax for defining an enum dataType
 */
// enum SoccerPosition { //  a finit set of possible Values, here is our enum namespace
//     Goalkeeper,
//     Fullback,
//     Center, // enum variant
//     Defence
// }
//
// fn next_player(position: SoccerPosition) { // enum is enumeration dataType
//     todo!()
// }
// fn main() {
//     let position = SoccerPosition::Defence; // player can only be in one one of these position at a time, our soccer program wont need to allow users to define new soccer positions
//     next_player(position);
// }







/*
    there is diffrent for enums in languages like java and c, which cant hold additional data.
    also enums and match expression work well together.
    we will see both of these in our example
 */


// we model our Clock dataType with 3 different kind of clock type
// enum Clock { // note that these enum variants are like tuple
//     Sundial(u8), // clock with just hour, its u8 becouse clock has not negative number
//     Digital(u8, u8), // clock with hour and minutes
//     Analog(u8, u8, u8),// Clock with hour, minutes, seconds
// }
//
// fn tell_time(clock: Clock) { // enum clock as parameter dataType
//     match clock {
//         Clock::Sundial(hours) => // because enum variant hold value , then pattern can access to that,
//             println!("It is about {} o'clock", hours),
//         Clock::Analog(hours, minutes, seconds) => {
//             println!(
//                 "It is {} minutes and {} seconds past {} o'clock",
//                 minutes, seconds, hours,
//             );
//         },
//         Clock::Digital(hours, minutes) =>
//             println!("It is {} minutes past {}", minutes, hours),
//     }
// }
//
// fn main() {
//     tell_time(Clock::Analog(9, 25, 45)); // pattern matching with predictable values
// }









