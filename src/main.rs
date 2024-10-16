use std::fs;
use funcs::encode_rsa::encode_rsa;
mod funcs;
fn main() {
    //从文件读取明文
    let file_path = "lab2-Plaintext.txt";
    //let file_path = "test2.txt";

    let plaintext = fs::read_to_string(file_path).unwrap_or(String::from(""));
    
    //转化为比特串，分组，每个分组对应的十进制数小于n，即分组长度小于log2n
    //所以要先生成密钥，得到n之后再进行分组。

    //加密函数主体
    let (n,e,d,phi_n,p,q) = funcs::key_gen::key_gen();
    let p_str = p.to_str_radix(10);
    let q_str = q.to_str_radix(10);
    let phi_n_str = phi_n.to_str_radix(10);
    let e_str = e.to_str_radix(10);
    let n_str = n.to_str_radix(10);
    let d_str = d.to_str_radix(10);

 
    let pub_key = funcs::key_gen::BigInt::
        parse_bytes(format!("{}{}",e_str,n_str).as_bytes(),10);    
    let pri_key = funcs::key_gen::BigInt::
        parse_bytes(format!("{}{}",d_str,n_str).as_bytes(),10);    

    //编码
    let encoded_text = funcs::base62::base62_encode(&plaintext);
    //分组的序列应该是一个vec, 里面的元素是4个十进制数字拼接形成的串（可计算）
    let encoded_cipher = encode_rsa(&encoded_text, &e, &n);

    //写入文件
    let result = fs::write("cipher.txt", encoded_cipher);
    //todo:错误处理
    //保存密钥
    
    let result_key = fs::write("pub_key.txt",&pub_key.unwrap().to_string());
    let result_key = fs::write("pri_key.txt",pri_key.clone().unwrap().to_string());
    let result_key = fs::write("p.txt",p_str);
    let result_key = fs::write("q.txt",q_str);
    let result_key = fs::write("phi_n.txt",phi_n_str);
    let result_key = fs::write("n.txt",n_str);
    let result_key = fs::write("e.txt",e_str);
    let result_key = fs::write("d.txt",d_str);


    //读取文件，解密
    let cipher = fs::read_to_string("cipher.txt").unwrap();
    println!("cipher :{}",cipher.clone());
    //解密函数
    let decoded_cipher = funcs::decode_rsa::decode_rsa(&cipher,&d,&n);
    println!("decoded_cipher :{},\n decoded_len:{}",decoded_cipher.clone(),decoded_cipher.len());
    let decoded_base62  = funcs::base62::base62_decode(&decoded_cipher);
    println!("decoded_base62 :{} \n decoded_base62_len:{}",decoded_base62.clone(),decoded_base62.len());
    let result = fs::write("dec.txt", &decoded_base62); 
}



