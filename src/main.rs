use rand::prelude::*;
use primes::{Sieve, PrimeSet};
use std::io::{self,Write};


/// Tworzenie kluczy RSA
/// 1) Znajdź dwie duże liczby pierwsze (mające np. po 128 bitów). 
/// Oznacz je jako p i q. Istnieją specjalne algorytmy generujące duże liczby pierwsze, 
/// które wykorzystują np. test Millera-Rabina.
///
/// 2)
/// Oblicz:
/// Ø = (p - 1) × (q - 1)
/// oraz
/// n = p × q
///
/// Liczby pierwsze p i q usuń, aby nie wpadły w niepowołane ręce. 
/// phi  to tzw. funkcja Eulera, n jest modułem.
///
/// 3) Wykorzystując odpowiednio algorytm Euklidesa znajdź liczbę e, która jest 
/// względnie pierwsza z wyliczoną wartością funkcji Eulera Ø (tzn. NWD(e, Ø) = 1) 
/// Liczba ta powinna również spełniać nierówność 1 < e < n . 
/// Nie musi ona być pierwsza lecz nieparzysta.
/// 4) Oblicz liczbę odwrotną modulo phi  do liczby e, czyli spełniającą równanie 
///
/// d × e  mod Ø = 1. Można to zrobić przy pomocy rozszerzonego algorytmu Euklidesa, 
/// który umieściliśmy w naszym artykule.
///
/// 5) Klucz publiczny jest parą liczb (e, n), gdzie e nazywa się publicznym wykładnikiem. 
/// Możesz go przekazywać wszystkim zainteresowanym.
///
/// 6) Klucz tajny to (d, n), gdzie d nazywa się prywatnym wykładnikiem. Klucz ten należy przechowywać pod ścisłym nadzorem. 

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

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}


fn main() {

    print!("Wprowadz liczbe do zaszyfrowania: ");
    io::stdout().flush().unwrap();

    let t = get_input().trim().parse::<i32>().unwrap();
    print!("\n\n");


    let k:RSA = klucze_rsa_random();
    println!("{:?}", k);

    print!("\n\n");


    // szyfrowanie
    let ciph = potega_modulo(t as i128, k.e, k.n);
    println!("ZASZYFROWANA: {:?}", ciph);

    // deszyfrowanie
    let deciph = potega_modulo(ciph, k.d, k.n);
    println!("ODSZYFROWANA: {:?}", deciph);

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn szyfrowanie_own_p_i_q() {
        let t = 123;
        let p = 13;
        let q= 11;
    
        let k:RSA = klucze_rsa(p,q);
        let ciph = potega_modulo(t, k.e, k.n);

        assert_eq!(ciph, 7)
    }

    #[test]
    fn deszyfrowanie_own_p_i_q() {
        let p = 13;
        let q= 11;
        let ciph = 7;
    
        let k:RSA = klucze_rsa(p,q);
        let deciph = potega_modulo(ciph, k.d, k.n);

        assert_eq!(deciph, 123)
    }

}
