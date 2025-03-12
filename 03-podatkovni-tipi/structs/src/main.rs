
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


struct GeometrijskoZaporedje {
    zacetni: i64,
    trenutni: i64,
    krat: i64
}

use GeometrijskoZaporedje as GZ;

impl GZ {
    fn new(a0: i64, q: i64) -> GZ {
        return GZ {
            zacetni: a0,
            trenutni: a0,
            krat: q
        }
    }
    fn next(&mut self) -> i64 {
        let a = self.trenutni;
        self.trenutni = a * self.krat;
        return a
    }
}

enum BinOperacija {
    Plus,
    Minus,
    Times,
    Power,
}

enum Izraz {
    Konstanta(u32),
    Operacija(Box<Izraz>, BinOperacija, Box<Izraz>),
}

impl Izraz {
    fn eval(&self) -> u32 {
        use crate::Izraz::Konstanta;
        use crate::Izraz::Operacija;
        use crate::BinOperacija::Plus;
        use crate::BinOperacija::Minus;
        use crate::BinOperacija::Times;
        use crate::BinOperacija::Power;

        fn pomozna(izraz: &Izraz) -> u32{
            match &izraz {
                &Konstanta(x) => *x,
                &Operacija(x, op, y) =>
                    match op {
                        Plus => pomozna(&x) + pomozna(&y),
                        Minus => pomozna(&x) - pomozna(&y),
                        Times => pomozna(&x) * pomozna(&y),
                        Power => pomozna(&x).pow(pomozna(&y)),
                }
        }

    }
    pomozna(&self)    
    }

    fn collect(&self) -> u32 {
        use crate::Izraz::Konstanta;
        use crate::Izraz::Operacija;

        fn pomozna(izraz: &Izraz) -> u32{
            match &izraz {
                &Konstanta(_) => 1,
                &Operacija(x, _, y) => pomozna(&x) + pomozna(&y)
    }
    }
    pomozna(&self)
    }   

    fn izpis(&self) -> String {
        use crate::Izraz::Konstanta;
        use crate::Izraz::Operacija;
        use crate::BinOperacija::Plus;
        use crate::BinOperacija::Minus;
        use crate::BinOperacija::Times;
        use crate::BinOperacija::Power;

        fn pomozna(izraz: &Izraz) -> String {
            match &izraz {
                &Konstanta(x) => format!{"{}", *x},
                &Operacija(x, op, y) =>
                    match op {
                        Plus => "(".to_owned() + &pomozna(&x) + ")" + " " +&String::from("+") + " " + "(" + &pomozna(&y) + ")",
                        Minus => "(".to_owned() + &pomozna(&x) + ")" + " " + &String::from("_") + " " + &pomozna(&y) + ")",
                        Times => "(".to_owned() + &pomozna(&x) + ")" + " " + &String::from("*") + " " +  "(" + &pomozna(&y) + ")",
                        Power => "(".to_owned() + &pomozna(&x) + ")" + &String::from("**")  + "(" + &pomozna(&y) + ")",
                } 
        }
    } 
    pomozna(&self)
    }

    }
fn main() {
    // let mut a = GZ::new(2, 4);
    // println!("{}", a.next());
let primer_1: Izraz = Izraz::Operacija(Box::new(Izraz::Konstanta(1)), BinOperacija::Plus, Box::new(Izraz::Operacija(Box::new(Izraz::Konstanta(2)), BinOperacija::Times, Box::new(Izraz::Konstanta(3)))));
let primer_2: Izraz = Izraz::Operacija(Box::new(Izraz::Operacija(Box::new(Izraz::Konstanta(1)), BinOperacija::Plus, Box::new(Izraz::Konstanta(2)))), BinOperacija::Times, Box::new(Izraz::Konstanta(3)));
let primer_3: Izraz = Izraz::Operacija(Box::new(Izraz::Operacija(Box::new(Izraz::Konstanta(1)), BinOperacija::Plus, Box::new(Izraz::Konstanta(2)))), BinOperacija::Plus, Box::new(Izraz::Konstanta(3)));
let primer_4: Izraz = Izraz::Operacija(Box::new(Izraz::Operacija(Box::new(Izraz::Konstanta(5)), BinOperacija::Power, Box::new(Izraz::Konstanta(2)))), BinOperacija::Plus, Box::new(Izraz::Operacija(Box::new(Izraz::Konstanta(3)), BinOperacija::Power, Box::new(Izraz::Konstanta(2)))));

println!("{}", primer_4.eval());
println!("{}", primer_4.collect());
println!("{}", primer_4.izpis());
}

