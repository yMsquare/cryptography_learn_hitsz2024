use num_bigint::BigInt;
use num_traits::{Num, ToPrimitive};

//接受密文字符串input密钥d，n（BigInt类型）返回解密后的字符串
pub fn decode_rsa(input : &String, d: &BigInt,n: &BigInt)-> String{
    let mut ret = String::new();
    //获取密文分组长度，设置为最大可能长度：log2N
    let group_size = log2_bigint(n).to_usize().unwrap();
    //密文字符串
    let cipher = input.clone();
    //对密文进行分组
    let grouped = group_by_size(&cipher, &group_size);
    
    for c in &grouped{
        //处理分组长度，若
        // let mut c_str = c.to_str_radix(10);
        // if c_str.len() < group_size{
        //     c_str = "0".repeat(group_size-c_str.len()) + &c_str;
        // }
        //计算模指数，转换成字符串，push进ret字符串里
        let mod_result = super::key_gen::mod_exp(&BigInt::from_str_radix(&c, 10).unwrap(), d, n).to_str_radix(10);
        let mut mod_result_str = mod_result.to_string();
        if mod_result_str.len()<4{
            mod_result_str = "0".repeat(4-mod_result_str.len())+ &mod_result_str;
        }
        ret.push_str(&mod_result_str);
    }
    //println!("decoded:{:?},len:{}",ret,ret.len());
    ret
}


// group_by_size takes the &string and a &usize to group the string,
// parse each grouped string to a BigInt, put them into a Vec.
// returns a Vec<BigInt>
// group_by_size接受&string和&usize的参数，将输入string字符串按照usize的长度进行分组。
// 将分组后的每个字符串被放进一个Vec（动态数组）中。
// 返回一个Vec〈String〉
fn group_by_size(text: &String,group_size:&usize)->Vec<String>{
    let encode_chars:Vec<_>= text.chars().collect();
    let group_size = group_size.to_usize().unwrap();//转化为usize类型
    let mut ret :Vec<String>  = Vec::new();
    // chunks: Returns an iterator over chunk_size elements of the slice at a time, 
    // starting at the beginning of the slice
    for chunk in encode_chars.chunks(group_size) {
        let chunk_str: String = chunk.iter().collect();
       // println!("grouped_size:{},\ngroupedlen:{},\ngrouped:{:?}",group_size,chunk_str.len(),chunk_str);
        ret.push(chunk_str);
    }
    ret
    
}

//计算log2n，为密文分组做准备。
pub fn log2_bigint(n : &BigInt)->u64{
    let mut cnt = 0u64;
    let mut temp = n.clone();
    // 使用右移（除以 2）的方式计算 log2
    while temp > BigInt::from(1) {
        temp = &temp >> 1; // 除以 2
        cnt += 1; // 计数
    }
    cnt
}