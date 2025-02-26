use std::time::{Duration, Instant};

fn time_it<F: FnOnce() -> R, R>(f: F) -> Duration {
    let start = Instant::now();
    f();
    start.elapsed()
}

fn on_stack() {
    // Narišite shemo spreminjanja sklada in kopice
    // Za vsako vrstico napiši, kolikokrat se v pomnilniku pojavi 13?
    let mut a = [13; 100]; // seznam se shrani na sklad; 100 krat 13
    let mut b = a; // ker seznam ne živi na kopici se ne more spremenit lastništvo, b ne more kazat na isti seznam kot a (samo en si eno zadevo lasti) => seznam se kopira; 200 krat 13
    let q = Box::new(13); // Isto kot Box::new(13); naredi se prostor na kopici, kjer je shranjena 13; 201 krat 13
    println!("{}", q);
    let r = *q; // * nam "odpakira" škatle
    let p = &r; // ta & je referenca na r; p samo kaže na r
    a[0] = 1; // mamo 200 krat 13, saj prvo pri a spremenimo na 1, b pa se ne spremeni
    {
        let c = &b;
        println!("{}", c[0]);
    }
    println!("{}", b[0]); // printa 13
    println!("{}", a[0]); // printa 1
    println!("{}", p); // print že odpakira škatle/reference; printa 13
    println!("{}", r); // printa 13
    // println!("{}", q); // Razloži, zakaj to ne deluje; q nima več lastništva!!!!
}

/// Napišite funkcijo `swap`, ki zamenja vrednosti dveh celoštevilskih spremenljivk.

fn swap(x: &mut i32, y: &mut i32) {
    let c = *x;
    *x = *y;
    *y = c;
}

fn test_swap() {
    let mut a = 13;
    let mut b = 42;

    println!("a: {}, b: {}", a, b);
    // Izpiše `a: 13, b: 42`.

   swap(&mut a, &mut b);

    println!("a: {}, b: {}", a, b);
    // Izpiše `a: 42, b: 13`.
}

/// Popravite zakomentiran del spodnje funkcije, da bo deloval
fn str_own() {
    let x = String::from("Hello world"); // ne vemo kolk placa rabmo tko da damo na kopico
    let y = &x;
    println!("{}, {}", x, *y);
}

/// Popravite brez uporabe funkcije `clone`
/// Namig: sklad in kopiranje na skladu - kodo lahko spremenite
fn str_own2() {
    let x = (1, 2, (), String::from("Hello world"));
    let y = &x;
    println!("{:?}, {:?}", x, *y);
}

/// Popravite spodnji dve funkciji, da bosta delovali

fn wrong() {
    let s = String::from("Hello World");
    print_str(&s);
    println!("{}", s);
}

fn print_str(s: &String) {
    println!("{}", s)
}

/// ------------------------------------------------------------------------------------------------
/// Popravite spodnjo funkcijo, da bo delovala
fn fn1() {
    let s = String::from("Hello ");

    let mut s1 = s;

    s1.push_str("World!");

    println!("Success! {}", s1);
}

/// ------------------------------------------------------------------------------------------------
/// Popravite spodnjo funkcijo, da bo delovala

fn fn2() {
    let x = Box::new(5);

    // // Popravite zgolj tukaj vmes
    let mut y = Box::new(42);
    // //
    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}

/// ------------------------------------------------------------------------------------------------

fn fn3() {
    let t = (
        String::from("hello"),
        String::from("world"),
        String::from("!"),
    );

    let _s = t.1;

    // Izpišite čim večji del t-ja.
    println!("{}{}{}", t.0, _s, t.2);
}

/// ------------------------------------------------------------------------------------------------

fn fn4() {
    let x = 5;
    // Izpišite naslov spremenljivke x
    println!("{:p}", &x)
}

/// ------------------------------------------------------------------------------------------------

fn fn5() {
    let x = 13;
    let y = &x;

    // Popravite spodnjo vrstico, da bo bo enakost držala
    assert_eq!(13, *y);
}

/// ------------------------------------------------------------------------------------------------

/// Popravite spodnjo funkcijo, funkcija `helper` se mora poklicati čim bolj učinkovito.
fn fn6() {
    let mut s = String::from("hello, ");

    helper(&s);

    println!("Success!");
}

// Te funkcije ne spreminjajte
fn helper(s: &String) {}

/// ------------------------------------------------------------------------------------------------

/// Popravite spodnjo funkcijo, funkcija `helper2` se mora poklicati čim bolj učinkovito.
fn fn7() {
    let mut s = String::from("hello, ");

    helper2(&mut s);

    println!("Success!");
}
// Te funkcije ne spreminjajte
fn helper2(s: &mut String) {
    s.push_str("world")
}

/// ------------------------------------------------------------------------------------------------

/// Pojasnite, zakaj spodnja koda ne deluje
fn fn8() {
    let mut s = String::from("hello, ");

    let p = &mut s;

    p.push_str("world");

    println!("Success! {}", p);
    println!("Success! {}", s); // s je bil kle sposojen kot immutable ko je že prej blo kot mutable
    s.push_str("!");
}

/// ------------------------------------------------------------------------------------------------
/// Pojasnite, zakaj spodnja koda ne deluje in jo popravite
/// Pojasnite tudi zakaj je popravek ok

fn fn9() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &r1;

    println!("{}, {}", r1, r2);

    println!("Success!");
}

/// ------------------------------------------------------------------------------------------------
fn fn10() {
    // // Popravite spodnjo vrstico
    let mut s = String::from("hello, ");

    helper3(&mut s);

    println!("Success!");
}

fn helper3(s: &mut String) {}

/// ------------------------------------------------------------------------------------------------

fn main() {
    // test_swap();
    // str_own();
    // str_own2();
    // wrong();
    // fn1();
    // fn2();
    // fn3();
    // fn4();
    // fn5();
    // fn6();
    // fn7();
}
