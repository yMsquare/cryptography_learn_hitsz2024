use funcs::encode_rsa::encode_rsa;
use num_traits::Num;
use std::fs;
use std::io::{self,  Write}; // 添加 Write trait 用于 flush 操作

mod funcs;

fn main() -> io::Result<()> {
    // 读取明文文件
    let plaintext = fs::read_to_string("lab2-Plaintext.txt").unwrap_or_default();

    println!("是否重新生成密钥？yes/no");
    let mut input0 = String::new();
    loop {
        let _ = io::stdin().read_line(&mut input0);
        let input0 = &input0.trim();

        if input0.eq_ignore_ascii_case("yes") {
            // 开始计时：生成密钥对
            let key_gen_instant = std::time::Instant::now();
            // 生成密钥对
            let (n, e, d, phi_n, p, q) = funcs::key_gen::key_gen();
            let key_gen_elapsed = key_gen_instant.elapsed();
            println!(
                "key_gen elapsed at :{} millis.",
                key_gen_elapsed.as_millis()
            );

            let pub_key = format!("{}{}", e.to_str_radix(10), n.to_str_radix(10));
            let priv_key = format!("{}{}", d.to_str_radix(10), n.to_str_radix(10));
            println!("p:{},q:{},phi_n:{},e:{},d:{}", p, q, phi_n, e, d);

            // 保存密钥和其他参数
            let keys = [
                ("pub_key.txt", pub_key),
                ("pri_key.txt", priv_key),
                ("phi_n.txt", phi_n.to_str_radix(10)),
                ("n.txt", n.to_str_radix(10)),
                ("e.txt", e.to_str_radix(10)),
                ("d.txt", d.to_str_radix(10)),
            ];

            for (filename, data) in keys.iter() {
                save_to_file(filename, data)?;
            }
            break;
        } else if input0.eq_ignore_ascii_case("no") {
            println!("跳过生成密钥，直接开始加密...");
            break;
        } else {
            println!("非法输入！请重新输入：");
        }
    }

    // Base62 编码明文
    let encoded_text = funcs::base62::base62_encode(&plaintext);

    // 开始计时：加密
    let encoding_instant = std::time::Instant::now();
    // 加密
    // 从文件中读取生成的密钥对
    let e_str = fs::read_to_string("e.txt").unwrap_or_default().trim().to_string();
    let n_str = fs::read_to_string("n.txt").unwrap_or_default().trim().to_string();
    let d_str = fs::read_to_string("d.txt").unwrap_or_default().trim().to_string();
    println!("{}",e_str);
    let e = num_bigint::BigInt::from_str_radix(&e_str,     10).unwrap();
    let n = num_bigint::BigInt::from_str_radix(&n_str,     10).unwrap();
    let d = num_bigint::BigInt::from_str_radix(&d_str,     10).unwrap();
    let encoded_cipher = encode_rsa(&encoded_text,&e,&n); 
    let encoding_elapsed = encoding_instant.elapsed();
    println!(
        "encoding elapsed at : {} millis.",
        encoding_elapsed.as_millis()
    );

    // 写入加密后的密文
    fs::write("cipher.txt", encoded_cipher)?;
    println!("Cipher written to cipher.txt");

    let mut input = String::new();
    println!("decode:解密 exit:退出程序");
    let _ = io::stdin().read_line(&mut input);
    match_input(&input);

    // 读取加密后的密文，进行解密
    let cipher = fs::read_to_string("cipher.txt")?;
    //println!("cipher: {}", cipher);

    // 开始计时：解密
    let decode_instant = std::time::Instant::now();
    // 解密流程
    let decoded_cipher = funcs::decode_rsa::decode_rsa(&cipher, &d, &n);
    let decode_elapsed = decode_instant.elapsed();
    println!(
        "decoded_cipher: {}, decoded_len: {}",
        decoded_cipher,
        decoded_cipher.len()
    );
    println!(
        "decoding elapsed at {} millis",
        decode_elapsed.as_millis()
    );
    // Base62 解码解密后的密文
    let decoded_base62 = funcs::base62::base62_decode(&decoded_cipher);
    println!(
        "decoded_base62: {}, decoded_base62_len: {}",
        decoded_base62,
        decoded_base62.len()
    );

    // 将解密后的文本写入文件
    fs::write("dec.txt", &decoded_base62)?;

    Ok(())
}

// 辅助函数：保存密钥到文件
fn save_to_file(filename: &str, data: &impl ToString) -> io::Result<()> {
    let mut file = fs::File::create(filename)?;
    writeln!(file, "{}", data.to_string())?; // 写入并换行
    file.flush() // 确保数据写入到文件中
}

fn match_input(input: &str) {
    let input = input.trim();
    if input.eq_ignore_ascii_case("exit") {
        std::process::exit(0);
    }
}
