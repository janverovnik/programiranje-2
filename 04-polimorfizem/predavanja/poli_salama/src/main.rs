use std::fmt::Display;
use std::cmp::PartialOrd;
use std::cmp::Ordering;

fn prestej<T> (v: &Vec<T>) -> i32 {
    let mut stevilo = 0;
    for _ in v {
        stevilo += 1;
    }
    stevilo
}

fn najvecji<T: PartialOrd> (v: &Vec<T>) -> Option<&T> {
    let mut najvecji = None;
    for x in v {
        match najvecji {
            None => 
            najvecji = Some(x),
            Some(m) => if m < x { najvecji = Some(x) }
        }
    }
    najvecji
}

enum Result<T, E> {
    Ok(T),
    Error(E)
}

#[derive(PartialEq)]
struct Tocka<T, O> {
    x: T,
    y: T,
    oznaka: O
}

impl<T, O> Tocka<T, O> {
    fn abscisa(&self) -> &T {
        &self.x
    }

    fn naredi_novo_tocko_z_oznako_drugega_tipa<P> (self, oznaka: P) -> Tocka<T, P> { // mutable referenci ne moremo spremenit tipa tko da kle "&mut self" ne gre
        Tocka {
            x: self.x,
            y: self.y,
            oznaka: oznaka
        }
    }
}

impl ToString for Tocka<f64, String> {
    fn to_string(&self) -> String {
        String::from("točka")
    }
}

impl<O> Tocka<f64, O> {
    fn absolutna_vrednost(&self) -> f64 {
        (&self.x.powi(2) + &self.y.powi(2)).sqrt()
    }
}

impl PartialOrd for Tocka<f64, String> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        None
    }
}

impl<T: ToString> ToString for Option<T> {
    fn to_string(&self) -> String {
        match self {
            None => String::from("None"),
            Some(x) => format!("Some({})", x.to_string())  
        }
    }
}
fn main() {
    let v1 = vec![1, 2, 3, 4, 5, 6];
    let n1 = prestej(&v1);
    println!("{}", n1);
    let v2 = vec![1., 3., 5., 7., 9.];
    let n2 = prestej(&v2);
    println!("{}", n2);
    let p1 = Tocka {x: 3.0, y: 4.0, oznaka: String::from("A")};
    let p2 = Tocka {x: 3.0, y: 5.0, oznaka: String::from("B")};
    let p = najvecji(&vec![p1, p2])
    println!("Največja točka je {}", p.to_string())
    
}
