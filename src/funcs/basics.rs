pub use num_bigint::BigInt;
use num_traits::Zero;

pub 
fn is_even(n: &BigInt) -> bool {
    n % 2 == BigInt::from(0)
}

// greatest 
pub fn gcd(a:&BigInt,    b:&BigInt)-> BigInt{
    if b.is_zero(){
       // println!("{}",a);
        a.clone()
    }
    else{
        gcd(b, &(a % b))
    }
}

// pub fn eulers_totient(input :&BigInt)->BigInt{
//   //  println!("eulers_totient");
//     let mut cnt:BigInt = 0.to_bigint().unwrap();
//     let mut i:BigInt  = 1.to_bigint().unwrap();
//     while i < *input {
//         if gcd(&i,input)==1.to_bigint().unwrap(){
//             cnt +=1;
//         }
//         i+=1;
//     }
//     cnt
// }