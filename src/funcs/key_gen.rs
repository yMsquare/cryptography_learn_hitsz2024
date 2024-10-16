pub use num_bigint::{BigInt, RandBigInt, ToBigInt};
pub use num_traits::{One, Zero};

pub fn key_gen() -> (BigInt, BigInt, BigInt, BigInt, BigInt, BigInt) {
    //   println!("hi");
    let low = BigInt::from(1000);
    let high = BigInt::from(10000);
    //generate p, q
    let p = generate_prime(&low, &high);
    let q = generate_prime(&low, &high);
    //  println!("geneating prime");

    let n = &p * &q;
    let phi_n = eulers_totient(&n);

    //select e
    let e = select_e(&BigInt::from(1), &phi_n);

    // culculate d:
    let result = extended_euclid(phi_n.clone(), e.clone());
    let mut d = result.unwrap().1;

    if d < BigInt::zero() {
        d = &d % &phi_n + &phi_n;
    }
    //return pub key and private key
    return (n, e, d, phi_n, p, q);
}

//Euler's Totient

pub fn eulers_totient(input: &BigInt) -> BigInt {
    //  println!("eulers_totient");
    let mut cnt: BigInt = 0.to_bigint().unwrap();
    let mut i: BigInt = 1.to_bigint().unwrap();
    while i < *input {
        if super::basics::gcd(&i, input) == 1.to_bigint().unwrap() {
            cnt += 1;
        }
        i += 1;
    }
    cnt
}

pub fn generate_prime(low: &BigInt, high: &BigInt) -> BigInt {
    let mut rng = rand::thread_rng();
    let low = low.to_bigint().unwrap();
    let high = high.to_bigint().unwrap();
    loop {
        let candidate: BigInt = rng.gen_bigint_range(&low, &high);
        if is_prime(&candidate) {
            return candidate;
        }
    }
}

fn select_e(low: &BigInt, high: &BigInt) -> BigInt {
    //   println!("slecting e");
    let e = generate_prime(&1.to_bigint().unwrap(), high);
    if super::basics::gcd(&e, &eulers_totient(high)) == 1.to_bigint().unwrap() {
        e
    } else {
        select_e(low, high)
    }
}

//Miller Rabin算法
fn is_prime(n: &BigInt) -> bool {
    //  println!("miller");
    if n == &BigInt::from(2) {
        return false;
    }
    if n < &BigInt::from(2) {
        return true;
    }
    if super::basics::is_even(n) {
        return false;
    }
    let mut q = n - 1.to_bigint().unwrap();
    let mut k = 0;
    //find k and q
    while super::basics::is_even(&q) {
        q = &q / 2;
        k += 1;
    }
    let mut rng = rand::thread_rng();
    for _ in 0..20 {
        // 随机选择 a，1 < a < n - 1
        let a = rng.gen_bigint_range(&BigInt::from(2), &(n - BigInt::from(1)));
        // 计算 a^q mod n
        let mut x = mod_exp(&a, &q, &n);
        if x == BigInt::one() || x == n - BigInt::one() {
            continue; // 可能是素数，跳到下一轮测试
        }
        let mut found = false;
        for _ in 0..(k - 1) {
            x = (&x * &x) % n;

            if x == n - BigInt::one() {
                found = true;
                break;
            }
        }
        if !found {
            return false; // 合数
        }
    }
    true
}

// fn mod_exp(a : &BigInt, b:&BigInt,p:&BigInt)-> BigInt{
//     let mut result = BigInt::one();
//     let mut b = b.clone();
//     let mut a  = a.clone();
//     while b > BigInt::zero(){
//         while &b%BigInt::from(2) == BigInt::zero(){
//             a = (&a * &a )% p;
//             b = b /2;
//         }
//         b -= 1;
//         result = (&a & &result) % p;
//     }
//     result
// }

// (a, exp, n)-> a^exp mod n
pub fn mod_exp(a: &BigInt, exp: &BigInt, n: &BigInt) -> BigInt {
    let mut base = a % n;
    let mut result = BigInt::one();
    let mut exp = exp.clone();

    while exp > BigInt::zero() {
        if &exp % 2.to_bigint().unwrap() == BigInt::one() {
            result = (result * &base) % n;
        }
        exp /= 2;
        base = (&base * &base) % n;
    }

    result
}

fn extended_euclid(a: BigInt, b: BigInt) -> Option<(BigInt, BigInt, BigInt)> {
    // println!("exetending euclid");
    let mut x = BigInt::from(1);
    let mut y = BigInt::from(0);
    let mut last_x = BigInt::from(0);
    let mut last_y = BigInt::from(1);
    let mut a = a;
    let mut b = b;

    while b != BigInt::from(0) {
        let q = &a / &b; // 商
        let remainder = &a % &b; // 余数
        a = b.clone();
        b = remainder;

        // 更新 x 和 y
        let temp_x = x.clone();
        x = last_x - &q * x.clone();
        last_x = temp_x;

        let temp_y = y.clone();
        y = last_y - &q * y.clone();
        last_y = temp_y;
    }

    Some((a, last_x, last_y))
}
