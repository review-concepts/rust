/*
    functions:
        - Syntax of defining a function
            fn name(param1:type1, ...) -> return_type {
                ...body...
            }
        - Syntax for calling a functions
            name(param1)
        - Return values from functions
 */




// ba yek mesale sade mikhahim tabero barrasi mikonim
// name az type slice string hast , current_age az noe u8 ya unsigned integer
// dar in measl return_type nadarim
fn next_birthday(name: &str, current_age: u8){
    let next_age = current_age + 1;
    println!("Hi {}, on your next birthday, you'll be {}", name, next_age)
}

// hala ye mesale sade dige ba return_type
fn square(num: i32) -> i32 {
    return num * num;
}

fn main() {
    // call functions
    // vaghti mige &str yani vorodi string ba double qoute hast
    next_birthday("ramin",31);

    // chon meghdari az tabe bargardone mishe ono dar yek motegharei baraye namayesh zakhire mikonam
    let sq = square(10);
    println!("square of 10 is : {}", sq)

}
