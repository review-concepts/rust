/*
    hadafemon az module1 ine ke sarian betonim darki karbordi be zabane rust peida konim,

    variables:
    -   dar inja mikhahim bebinim syntaxe variable dar rust be che sorate
    -   mutability yani chi
    -   variable type ha ro ham beshnasim
 */


// khili sade shoro mikonim bebinim syntax variable be che sorate
//
fn main() {
    let a = 5;
    let b = 10;
    let c = a + b;
    println!("a + b = {}", c);
}

/*
    moteghayerha dar rust taghirnapazir hastand , yani chi?
    yani vaghti meghdari behesh dade beshe dar tole barname nemitonim meghdare ono taghir bedim va ba errore cannot assign twice to immutable variable movajeh mishim
    magar inke serahatan mut ro ghabl az esme moteghayer ezafe konim
 */
// fn main() {
//     let mut a = 5;
//     a +=1;
//     println!("a = {}", a);
// }

/*
    dar sorate niaz dar rust mishe dasti datatype haro ezafe kard
 */
// fn main() {
//     let a:i32 = 5;
//     let b:i32 = 10;
//     let c:i32 = a + b;
//     println!("a + b = {}", c);
// }