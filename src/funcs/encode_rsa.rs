use num_bigint::BigInt;
use num_traits::{Num, ToPrimitive};
use super::key_gen;
// 将编码后的明文进行分组并加密。
pub fn encode_rsa(encoded_text :&String, e:&BigInt, n:&BigInt)->String{
    let mut ret = String::new();
    let grouped = group_by_four(encoded_text);
    let grouped_size = super::decode_rsa::log2_bigint(n);
    println!("grouped cipher after encoding:{:?}",grouped);
    //分组后就可以加密了吧？
    for m in grouped{
        let tmp = key_gen::mod_exp(&m,e,n);
        let mut tmp_str = tmp.to_str_radix(10);
        //填充密文分组长度。
        if tmp_str.len() < grouped_size.to_usize().unwrap(){
            tmp_str = "0".repeat(grouped_size.to_usize().unwrap()-tmp_str.len()) + &tmp_str;
        }
        ret.push_str(&tmp_str);
    }
    //分组长度要固定吧？log2n
    ret
}

fn group_by_four(text: &String)->Vec<BigInt>{
    let mut encode_chars:Vec<_>= text.chars().collect();
    let mut ret :Vec<BigInt>  = Vec::new();
    println!("the len of cipher:{}",encode_chars.len());
    //缺少字符，人为添加一个两位数的。解密时需要删除
    if encode_chars.len() % 4 != 0{
       encode_chars.push('6');
       encode_chars.push('2');
    }
    println!("the len of cipher pushed:{}",encode_chars.len());
    //4个一组，转化为i32再转化为BigInt便于后续的加密计算。
    for c in encode_chars.chunks(4){
        let mut tmp = String::new();
        for i in c{
            tmp.push(*i);
        }
        ret.push(BigInt::from_str_radix(&tmp, 10).unwrap());//unwrap
    }
    ret
}