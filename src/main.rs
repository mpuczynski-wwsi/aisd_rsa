use rand::prelude::*;
use primes::{Sieve, PrimeSet};


#[derive(Debug)]
struct RSA {
    e:i128,
    d:i128,
    n:i128,
}

fn nwd( mut a: i128, mut b: i128) -> i128 {
    let mut t:i128;
    while b!=0 {
        t=b;
        b=a%b;
        a=t;
    }
    return a;
}

fn potega_modulo(a:i128, w:i128, n:i128) -> i128{
    let mut q = w;
    let mut pot = a;
    let mut wyn = 1;

    while q > 0 {
        if q % 2 > 0 {
            wyn = (wyn * pot) % n;
        }
        pot = (pot * pot) % n;
        q /= 2;
    }
    wyn


}

fn odwrotnosc_modulo(a:i128, n:i128) ->i128{
    let mut t:i128;
    let (mut p0, mut p1, mut a0, mut n0) = (0,1,a,n);
    let mut q:i128 = n0 / a0;
    let mut r:i128 = n0 % a0;
    while r > 0 {
        t=p0-q*p1;
        if t>=0{
            t=t%n;
        } else {
            t=n-((-t)%n);
        }
        p0=p1;
        p1=t;
        n0=a0;
        a0=r;
        q=n0/a0;
        r=n0%a0;
    }
    return p1;
}

fn klucze_rsa_random() -> RSA{
    // const N:usize = 10;
    // const LICZBY_PIERWSZE: [i128;N] = [11,13,17,19,23,29,31,37,41,43];
    let mut p:i128;
    let mut q:i128;
    let mut rng = rand::thread_rng();
    let mut pset = Sieve::new();
    loop {
        let l1 = rng.gen_range(100_000, 1_000_000);
        let l2 = rng.gen_range(100_000, 1_000_000);
    
        let r1 = pset.find(l1);
        let r2 = pset.find(l2);
        p = r1.1 as i128;
        q = r2.1 as i128;
        // p = LICZBY_PIERWSZE[rng.gen_range(0, N)];
        // q = LICZBY_PIERWSZE[rng.gen_range(0, N)];
        if p!=q { break; }
    }
    println!("{} {}", p,q);

    let phi:i128 = (p - 1) * (q - 1);
    let n:i128 = p*q;

    let mut e:i128 = 3;
    while nwd(e,phi) != 1 {
        e+=2;
    }
    let d = odwrotnosc_modulo(e, phi);

    RSA {
        e: e,
        d: d,
        n: n,
    }
}

fn klucze_rsa(p:i128,q:i128) -> RSA{
    let phi:i128 = (p - 1) * (q - 1);
    let n:i128 = p*q;

    let mut e:i128 = 3;
    while nwd(e,phi) != 1 {
        e+=2;
    }
    let d = odwrotnosc_modulo(e, phi);
    RSA {
        e: e,
        d: d,
        n: n,
    }
}


fn main() {



    let p = 13;
    let q= 11;

    let k1:RSA = klucze_rsa(p,q);
    let k2:RSA = klucze_rsa_random();
    println!("{:?}", k1);
    println!("{:?}", k2);

    let t = 123;
    // szyfrowanie
    let ciph1 = potega_modulo(t, k1.e, k1.n);
    println!("{:?}", ciph1);
    let ciph2 = potega_modulo(t, k2.e, k2.n);
    println!("{:?}", ciph2);






    // deszyfrowanie
    let deciph1 = potega_modulo(ciph1, k1.d, k1.n);
    println!("{:?}", deciph1);
    let deciph2 = potega_modulo(ciph2, k2.d, k2.n);
    println!("{:?}", deciph2);

}
