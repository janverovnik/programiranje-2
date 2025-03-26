// Napišite različne funkcije, ki kot argument sprejmejo zaprtje (closure) in ga pokličejo z različnimi argumenti, ki ustrezajo sledečim ocaml tipom:
// Tip zaprtja naj bo čim bolj splošen (Fn, FnOnce, FnMut).
//  apply_int: (int -> int) -> int -> int
//  apply: ('a -> 'b) -> 'a -> 'b
//  apply2: ('a -> 'a -> 'b) -> 'a -> 'a -> 'b
//  map: ('a -> 'b) -> 'a list -> 'b list  (Uporabite Vec<T> namesto list, predpostavite, da funkcija ne spremeni elementov seznama)
//  ponavljaj: int -> ('a -> 'a) -> 'a -> 'a // Ponovi funkcijo n-krat
//  filter: ('a -> bool) -> 'a list -> 'a list // Vrne seznam elementov, ki zadoščajo pogoju - uporabite Vec<T> namesto list in že vgrajeno funkcijo filter

fn apply_int(f: &dyn Fn(i64) -> i64, x: i64) -> i64 {f(x)}
fn apply<T, H>(f: &dyn Fn(T) -> H, x: T) -> H {f(x)}
fn apply2<T, H>(f: &dyn Fn(T, T) -> H, x: T, y: T) -> H {f(x, y)}
fn map<T, H>(f: &dyn Fn(T) -> H, v: Vec<T>) -> Vec<H> {v.into_iter().map(f).collect()}
fn ponavljaj<T>(n: u32, f: &dyn Fn(T) -> T, mut x: T) -> T { 
    for _ in 1..n {x = f(x)};
    x
}
fn filter<T>(f: &dyn Fn(&T) -> bool, v: Vec<T>) -> Vec <T> {v.into_iter().filter(f).collect()}

// Vzemite zaporedja iz prejšnjih vaj in naredite nov objekt, ki sprejme zaporedje in ga naredi iterabilnega

struct AritmeticnoZaporedje {
    zacetni: i64,
    trenutni: i64,
    plus: i64
}

use AritmeticnoZaporedje as AZ;

impl AZ {
    fn new(zacetni: i64, dodatek: i64) -> AZ {
        return AZ {
            zacetni: zacetni,
            trenutni: zacetni,
            plus: dodatek
        };
    }  
    fn next(&mut self) -> i64 {
        let a = self.trenutni;
        self.trenutni += self.plus;
        return a;
    }
    fn n_th(&mut self, n: i64) -> i64 {
        return self.zacetni + n * self.plus;
    }
    fn reset(&mut self) {
        self.trenutni = self.zacetni;
    }
    fn current(&mut self) -> i64 {
        return self.trenutni;
    }
    fn sum(&mut self, n: i64) -> i64 {
        let mut vsota = 0;
        for i in 0..n {
            vsota += self.n_th(i);
        }
        return vsota
    }
}

impl AZ {
    fn vsota(a: AZ, b: AZ) -> AZ {
        return AZ {
            zacetni: a.zacetni + b.zacetni,
            trenutni: a.trenutni + b.trenutni,
            plus: a.plus + b.plus
        }
    }
    fn produkt(a: AZ, b: AZ) -> AZ {
        return AZ {
            zacetni: a.zacetni * b.zacetni,
            trenutni: a.trenutni * b.trenutni,
            plus: a.plus * b.zacetni + b.plus * a.zacetni + a.plus * b.plus
        }
    }
}

// Iteratorji

// Napišite funkcijo, ki sprejme vektor XYZ in s pomočjo iteratorja naredi W 
// števil in izpiše vsako v svojo vrstico 
// nizov in izpiše njihove dolžine 
// nizov in vrne vsoto njihovih dolžin 
// vektor parov (i32, i32) in vrne vsoto njihovih pozitivnih produktov 
// dva vektorja <i32> in vrne vektor, ki vsebuje vsote parov 
// dva vektorja <i32> in vrne vsoto poparjenih pozitivni produktov s pomočjo ene izmed prejšnjih nalog 
// vektor Option<T> in izpiše vse T-je 
// vektor Option<T> in vrne število Some-ov 
// odfiltrira števila deljena s 3 










// Dopolnite spodnjo funkcijo, da vrne niz, kjer so vse prve črke posameznih besed velike
// ["Just,", " ", "hello", " ", "world", "!"] -> "Just, Hello World", "!"
// pub fn capitalize_words_string(words: &[&str]) -> String {
//     panic!("Not implemented");
// }
// Napišite funkcijo `fakulteta`, ki izračuna fakulteto števila n. Uporabite iteratorje (torej brez lastne for zanke, rekurzije)
// Namig: fold, reduce, `..`...

// -------------------------------------------------------------------------------------------------
// Dodatno:
// Koda vzeta iz googlvih rust vaj:
// Vse se da lepo narediti samo z iteratorji (brez indeksov, brez for zank, brez mutabilnosti)
/*
/// Calculate the differences between elements of `values` offset by `offset`,
/// wrapping around from the end of `values` to the beginning.
///
/// Element `n` of the result is `values[(n+offset)%len] - values[n]`.
fn offset_differences<N>(offset: usize, values: Vec<N>) -> Vec<N>
where
    N: Copy + std::ops::Sub<Output = N>,
{
    unimplemented!()
}

#[test]
fn test_offset_one() {
    assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
    assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
    assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
}

#[test]
fn test_larger_offsets() {
    assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
    assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
    assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
    assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
}

#[test]
fn test_custom_type() {
    assert_eq!(
        offset_differences(1, vec![1.0, 11.0, 5.0, 0.0]),
        vec![10.0, -6.0, -5.0, 1.0]
    );
}

#[test]
fn test_degenerate_cases() {
    assert_eq!(offset_differences(1, vec![0]), vec![0]);
    assert_eq!(offset_differences(1, vec![1]), vec![0]);
    let empty: Vec<i32> = vec![];
    assert_eq!(offset_differences(1, empty), vec![]);
}



*/
fn main() {
    let f_1 = |x: i64| -> i64 {x * x};
    println!("{}", ponavljaj(6, &f_1, 2));

    let f_2 = |x: &u64| -> bool {*x % 2 == 0};
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 42];
    println!("{:?}", filter(&f_2, v));
}