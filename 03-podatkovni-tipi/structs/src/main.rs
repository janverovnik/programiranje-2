
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



fn main() {
    let mut a = GZ::new(2, 4);
    println!("{}", a.next());

}

