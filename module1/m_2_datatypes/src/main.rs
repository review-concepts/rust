/*
    dar inje ye morore sari mikhahim dar dataType haye zabane rust dashte bashim
    age bekhahim taghsimbandi dashte bashim dataTypeha be do bakhshe kolli taghsim mishan
    -   core language dataType : khodesh be do bakhshe dige mitone taghsim beshe
        -   Simple:
            -   Boolean
            -   Integer
            -   Floating point
            -   Charachter
        -   Compound:
            -   Tuple
            -   Array
            -   Slice
    -   custom dataType : ino badan barrasi mikonim
    
 */








/*
    Bool: dataType Boolean dar rust ba Bool shenakhte mishavad
    true or false: do ta value migirad true ya false
    Used in control flow: mafhom an omoman dar control jaryane barname karbord darad meslan to shart ha
 */

// fn main() {
//     let a = true;
//     let b = false;
//
//     // be println! tavajo dashte bashid har ja didid ke ! omade yani macro ke badan barrasish mikonim
//     if a {
//         println!("a is true");
//     }
//     if b {
//         println!("b is true!")
//     }
// }








/*
    numbers without decimal points
    signed and unsigned - iSize and uSize : integer dataType be do sorat hastan ba alamat va bi alamat dar ba almat (iSize) az 8 bit , avalin bite samte chap , bit alamat hast va baze -128 ta +128 ra poshesh midahad dar bi alamat tamame har 8 bit arzeshe raghami hastan va baze 0 ta 255 ra poshesh midahad
    used for indexes, counts: nokteii ke vojod dare ine ke midonim az uSize ya dataType bi alamat baraye shomaresh dar loop ha va hamchenin dar dastrasi index arrayeha estefade mishe
 */
// fn main() {
//     // dataType ro mitonim khodemon moshakhas konim ama age moshakhas nakonim rust betore pishfarz an dataTypii ro entekhab mikone ke kamtarin hafezero eshghal mikone
//     let num_i = 32;
//     let num_i: i32 = 32;
//     let num_i: i64 = 32;
//
//     // dar rust betore pishfarz age dataType baraye float moshakhas nakonim dataTypesh ro f64 gharar mide
//     let float_num = 3.14;
//
//
//     // itemhaye arraye zir mitonan manfi bashan ya mosbat pas az no iSize hastan
//     let a = [100,200,300];
//     // index a[0], 0 az no uSize hast chon index manfi dar arraye nadarim
//     let b= a[0];
//
// }










/*
    Char: dataType charachter dar rust ba Char shenakhte mishavad
    single qoute not double qoute,
    unicode scalar value : be gheir az ASCII, unicode ham migire yani mitonim horofe farsi ham benevisim berahati
 */
// fn main() {
//     let c = 'a';
//     let d = 'س';
// }






/*
    Tuple
    Group multiple values into one dataType : dar tuple hadaf group kardane value ba dataTypehaye mokhtalef to ye dataType hast
    Values dont have to be the same type
 */
// fn main() {
//     // value ba typehaye mokhtalefi ro ba tuple group kardim
//     let tup = (1,'c',true); // be in tarif migim tuple
//
//     // barye dastrasi be valueha be shive zir amal mikonim
//     let fist = tup.0;
//     let second = tup.1;
//     println!("The first item is {}", fist);
//     println!("The second item is {}", second);
//
//     // tuple ye built-in compund structe hast
//     // mitonim deStructure ham bokonim yani groupe tuple ro ajzash tabdil mikonim
//     // baraye mesal tuple tup balaro mikhahim deStructure bokonim
//     let (x,y,z) = tup;
//     println!("x: {}, y: {}, z: {}",x,y,z);
// }










/*
    Array:
    Group multiple values into one value : dar arraye hadaf group kardane valuehaye mokhtalefe
    Values must be the same type : dat tuple didim ke dataType ye group mitonan mokhtalef bashand ama dar arraye hatman yek dataType bayad bashe
 */
// fn main() {
//
//
//     let mut a = [0.0, 3.14,-8.7928];
//     a[0] = 45.5; // ham mitonim be arraye access peida konim va ham modify konim
//
// }










/*
    Slice:
    reference to a subset of data in another data sturcture : refrence hamon ampersand & ast , subset of data yani zir majomeye data az sakhtare digeii, dar mesal bebinim ta rahhatar motevajeh beshim
 */
// fn main() {
//     // array data sturcture
//     let a = [100, 200, 300];
//
//     // reference to a subset of array structer
//     let b = &a[2..3]; // behemon arraye bargardonde
//     println!("{:?}", b);
// }
