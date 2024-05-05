use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    // 随机数种子
    let mut  rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        // 添加候选值
        chars.extend_from_slice(UPPER);
        // 从upper候选值中挑选一位
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }

    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
    }
    for _ in 0..(length - password.len() as u8 ){
        let c = chars.choose(&mut rng).expect("chars won't be empty in this context");
        password.push(*c)
    }
    password.shuffle(&mut rng);
    let pass_str = String::from_utf8(password)?;
    println!("{}", pass_str);
    // 密码的强度
    let estimate = zxcvbn(pass_str.as_str(), &[]).unwrap();
    // 使用eprintln打印不会输出到标准输出中
    eprintln!("level: {}", estimate.score()); // 3
    Ok(())
}