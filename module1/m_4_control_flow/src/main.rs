/*
    dar rust control flow be bakhshhaye zir taghsim mishan:

    if / else if / else
    loop / while / for
    match

    hamontor ke mibinid switch nadarim ama mishe gofti beja on chizi be esme match vojod dare ke barrasi mikonim

 */








/*
    if / else if / else
 */
// fn main() {
//     discount(10);
// }
//
// fn discount(day_of_month: u8){
//     // nokteii ke dar in if else vojod dare ineke chon darim be yek moteghayer entesab midim badane if va ya badane else hardo az yek type bayad bashand dar inja i32
//     let amount = if day_of_month % 2 == 0 {
//         50
//     } else {
//         10
//     };
//
//     println!("Your discount is {}", amount);
// }
//










/*
    loop:
        The loop keyword lets you specify a block of code that should be run forever
 */
// use: import items from other modules,
// use std::io;
// fn main() {
//     loop {
//         println!("Whats's the sercret word?");
//         // moteghayer word ro baraye daryafte vorodi reshte az keybord sakhtim
//         let mut word = String::new();
//         // dar hale hazer ziad nemikhaim dar khate zir amigh beshim, faghat bedonim ke dare vorodi ro az keybord migire va be moteghayer word mifreste
//         io::stdin().read_line(&mut word).expect("failed to read line");
//         // trim() whitespace haye kenare word ro bar midare
//         if word.trim() == "rust" {
//             break; // break of loop
//         }
//     }
//     println!("you know the secret word!")
// }











/*
    While:
    an expression that evaluates to true or false then curly brackets and a block of code
    lets rewrite loop example via while
 */
// use std::io;
// fn main() {
//     let mut word = String::new();
//     // che sharti bayad vojod dashte bashe ta az hamon ebteda morede barrasi gharar begire?
//     // che sharti benevisim ke baraye avalin bar ham azamon betone soal whats the secret word ro ham azamon beporse?
//     while word.trim() != "rust" {
//         word = String::new();
//         println!("whats the secret word?");
//         io::stdin().read_line(&mut word).expect("Failed to read line");
//     }
//
//     println!("you know the secret word!")
// }



/*
    for:
    run some code for each item in a collection
    dar zabane c ma niaz dashtim index haye yek collection ro modiriat bokonim, vali dar inja in negarani vojod nadare
 */
// fn main() {
//     // baraye har i dar collection [1..11) code haye body ro ejra kon
//     for i in 1..11 { // [1,11) range ya hamon baze hast,
//         println!("now serving number {}", i)
//     }
// }









/*
    match:
    match mafhome taghriban jadidie ke momkene shoma dar zabanhaye dige in sysntax ro nadide bashin
    be if/if-else/else va switch/case sheabahat dare ama yek seri khosisathaii dare ke kare maro to control flow rahat mikone
    ghabl az inke mesal bezanam be in nokte tavajoh konid ma baraye match niaze tamame halatharo beshnasim
    masalan dar yek tas midonim 6 halat vojod dare
 */
// fn main() {
// // dar inja amalakarde match shabihe if-else hast
//     let x = 3;
//     match x {
//         1 => println!("One is the loneliest number."), // halate aval: x ba pattern 1 match hast?
//         2 => println!("Two's company"), // halate dovom: aya x ba pattern 2 match hast?
//         3 => println!("Three's a crowd"),// halate sevoom: aya x ba pattern 3 match hast?
//         _ => println!("Some other number"),// sayere halathaa
//     }
// }




/*
     match on tuple : hala mikhahim mesale jalebtari az pattern matching bezanim,
 */
// fn main() {
//
//     let die1 = 1;
//     let die2 = 5;
//
//     match (die1, die2) {
//         (1, 1) => println!("Snake eyes! Go back to the beginning."),
//         (5, _) | (_, 5) => { // underscore in the tuple is mean to ignore any other value
//             // dar pattern hamon ke mibinid ma baraye inke chandin line dastor dashte bashim ke bade match shodan ejra beshan darone block mineviseshon
//             println!("You rolled at least one 5!");
//             println!("Move and then roll again!");
//         },
//         _ => println!("Move your piece!"), // an underscore that will match any value for the whole tuple that hasnt already been matched
//     }
// }






/*
    dar nazar begirid hamontor ke dar bala bayan shod ma mitonim tamame halathaye momkene ro dar pattern dar nazar begirim ta manteghe match barname monasebe ba an be jaryan biofte
 */
// fn main() {
//     let is_confirmed = true;
//     let is_active = false;
//
//     match (is_confirmed, is_active) {
//         (true, true) => println!("Your account is in good standing."),
//         (false, true) => println!("You need to confirm your account!"),
//         (false, false) => println!("This account will be deactivated."),
//         (true, false) => println!("Your account is blocked."),
//         // _ => {}  age halatii momkene vojod dashte bashe ama ma natonim to manteghe barname pish bini bokonim az in estefade mikonim va ono be ohde zabane rust mizarim
//     }
// }
