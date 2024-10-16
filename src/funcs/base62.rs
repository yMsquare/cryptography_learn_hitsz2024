// 为明文进行base62编码时，以4个数字一组为单位进行编码
pub fn base62_encode(input: &str)->String{
    let mut ret = String::new();
    for char in input.chars(){
        let encoded = match char{
            '0'..='9' =>{
                format!("{:02}",char as u8 - b'0')
            }
            'a'..='z'=>{
                format!("{:02}",char as u8 - b'a'+ 10)
            }
            'A'..='Z'=>{
                format!("{:02}",char as u8 - b'A' +36)
            }
            _ =>{
                String::from("")
            }
        };
        ret.push_str(&encoded);
    }
    return ret;
}

//pub fn base62_decode(input: &str)->String{
//    let mut ret = String::new();
    // let pairs: Vec<String> = input
    // .chars() // 获取字符迭代器
    // .collect::<Vec<_>>() // 收集为向量
    // .chunks(2) // 每两字符一组
    // .map(|chunk| chunk.iter().collect::<String>()) // 将每组字符转换为字符串
    // .collect();
    // for pair in pairs{
    //     let decoded = match pair {
    //         =>{

    //         }
    //     };
//     let mut padded =String::new();
//     //密文不足4位，前面补0
//     if input.len() < 4{
//         for _ in 0..=input.len(){
//             padded.push('0');
//         }
//     }
//     padded.push_str(input);
//     //密文为奇数
//     // }
//     return ret;
// }

pub fn base62_decode(input: &str) -> String {
    let mut ret = String::new();

    // 处理输入的长度
    let mut padded = input.to_string();
    // 密文不足4位，前面补0
    while padded.len() % 4 != 0 {
        padded.insert(0, '0');
    }

    // 将输入分成每4个字符一组
    let chars: Vec<_> = padded.chars().collect();
    for chunk in chars.chunks(4) {
        if chunk.len() == 4 {
            let mut decoded_chars = vec![];

            for pair in chunk.chunks(2) {
                if let [first, second] = pair {
                    // 将字符转换为对应的数字
                    let num = match *first {
                        '0'..='9' => (*first as u8 - b'0') as usize,
                        'a'..='z' => (*first as u8 - b'a' + 10) as usize,
                        'A'..='Z' => (*first as u8 - b'A' + 36) as usize,
                        _ => continue, // 非法字符，跳过
                    } * 10 + match *second {
                        '0'..='9' => (*second as u8 - b'0') as usize,
                        'a'..='z' => (*second as u8 - b'a' + 10) as usize,
                        'A'..='Z' => (*second as u8 - b'A' + 36) as usize,
                        _ => continue, // 非法字符，跳过
                    };

                    // 根据计算出的数字添加到解码字符中
                    decoded_chars.push(num);
                }
            }

            // 将解码的数字转换为字符并加入结果
            for &num in decoded_chars.iter() {
                if num < 10 {
                    ret.push((b'0' + num as u8) as char);
                } else if num < 36 {
                    ret.push((b'a' + (num - 10) as u8) as char);
                } else if num < 62{
                    ret.push((b'A' + (num - 36) as u8) as char);
                }
                else{
                    continue;
                }
            }
        }
    }
    ret
}