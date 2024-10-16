use num_bigint::BigInt;
use num_traits::{Num, ToPrimitive};
use super::key_gen;
// 将编码后的明文进行分组并加密。
pub fn encode_rsa(encoded_text :&String, e:&BigInt, n:&BigInt)->String{
    let mut ret = String::new();
    let grouped = group_by_four(encoded_text);
    let grouped_size = super::decode_rsa::log2_bigint(n);
    println!("grouped cipher after base62 encoding:{:?}",grouped);
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
    //缺少字符，意味着最后一组可能只有1个字母（对应base62编码下的2位十进制数）
    //需要人为添加一个两位十进制数凑齐4位(不属于base62编码集，所以这个十进制数要大于61)。
    //解密后解码base62时需要删除这个人为添加的字符（或者直接在解码base62的函数里无视这种非法字符）
    if encode_chars.len() % 4 != 0{
       encode_chars.push('6');
       encode_chars.push('2');
    }
    println!("the len of padded cipher:{}",encode_chars.len());
    //4个一组，转化为BigInt便于后续的加密计算。
    for c in encode_chars.chunks(4){
        let mut tmp = String::new();
        for i in c{
            tmp.push(*i);
        }
        ret.push(BigInt::from_str_radix(&tmp, 10).unwrap());//unwrap
    }
    ret
}