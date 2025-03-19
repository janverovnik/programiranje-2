
#[derive(PartialEq, Debug)]
struct AritmeticnoZaporedje<T: Copy + Clone + PartialOrd + std::ops::Add<Output = T> + std::ops::AddAssign + std::ops::Mul<Output = T>> {
    zacetni: T,
    trenutni: T,
    plus: T
}

use AritmeticnoZaporedje as AZ;

impl<T: Copy + Clone + PartialOrd + std::ops::Add<Output = T> + std::ops::AddAssign + std::ops::Mul<Output = T>> AZ<T> {
    fn new(zacetni: T, dodatek: T) -> AZ<T> {
        let a = zacetni.clone();
        return AZ {
            zacetni: a,
            trenutni: zacetni,
            plus: dodatek
        };
    }  
    fn next(&mut self) -> T {
        let a = self.trenutni.clone();
        self.trenutni += self.plus.clone();
        return a;
    }
    fn n_th(&mut self, n: i64) -> T {
        let mut a = self.zacetni;
        for _ in 1..n{
            a += self.plus; 
            
        }
        return a
    }
    fn reset(&mut self) {
        self.trenutni = self.zacetni.clone();
    }
    fn current(&mut self) -> T {
        return self.trenutni.clone();
    }
    fn sum(&mut self, n: i64) -> T {
        let mut vsota = self.zacetni.clone();
        for i in 1..n {
            vsota += self.n_th(i);
        }
        return vsota
    }
    fn vsota(a: AZ<T>, b: AZ<T>) -> AZ<T> {
        return AZ {
            zacetni: a.zacetni + b.zacetni,
            trenutni: a.trenutni + b.trenutni,
            plus: a.plus + b.plus
        }
    }
    fn produkt(a: AZ<T>, b: AZ<T>) -> AZ<T> {
        return AZ {
            zacetni: a.zacetni * b.zacetni,
            trenutni: a.trenutni * b.trenutni,
            plus: a.plus * b.zacetni + b.plus * a.zacetni + a.plus * b.plus
}
}
}


trait Zaporedje<T> {
    fn name(&self) -> String;
    fn start(&self) -> T;
    fn k_th(&self, k: i64) -> Option<T>; 
    fn contains(&self, item: T) -> bool;
}

struct KonstantnoZaporedje<T> {
    c: T
}
use KonstantnoZaporedje as KZ;

impl<T> KZ<T> {
    fn new(a: T) -> KZ<T> {
        KZ {c: a}
    }
}

impl<T: Copy + std::cmp::PartialEq> Zaporedje<T> for KZ<T> {
    fn name(&self) -> String {
        format!("Konstantno zaporedje")
    }
    fn start(&self) -> T{
        return self.c
    }
    fn k_th(&self, k: i64) -> Option<T> {
        return Some(self.c)
    }
    fn contains(&self, a: T) -> bool{
        return self.c == a
    }
}

impl<T: Copy + Clone + PartialOrd + std::ops::Add<Output = T> + std::ops::AddAssign + std::ops::Mul<Output = T> + std::fmt::Debug + std::ops::Rem> Zaporedje<T> for AZ<T> {
    fn name(&self) -> String {
        format!("AZ({:?}, {:?})", self.zacetni, self.plus)
    }
    fn start(&self) -> T {
        return self.zacetni
    }
    fn k_th(&self, k: i64) -> Option<T> {
        let mut a = self.zacetni;
        for _ in 1..k {
            a += self.plus 
        }
        return Some(a)
    }
    fn contains(&self, a: T) -> bool{
        return false
    }
}


fn main() {
    println!("Hello, world!");
}
