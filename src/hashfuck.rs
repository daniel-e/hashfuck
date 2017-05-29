extern crate crypto;

use std::iter::FromIterator;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use self::crypto::sha2::Sha512;
use self::crypto::md5::Md5;

fn split_into_hex_bytes(s: &str) -> Vec<String> {
    s
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|x| String::from_iter(x.iter()))
        .collect()
}

fn hex_to_brainfuck(hash: &str) -> String {
    let bf_ops = vec![">", "<", "+", "-", ".", ".", "[", "]"];

    split_into_hex_bytes(hash).iter()
        .map(|hex| bf_ops[usize::from_str_radix(hex, 16).expect("invalid hash") % 8])
        .collect()
}

fn contains_ff(s: &str) -> bool {
    split_into_hex_bytes(s).into_iter().any(|x| x == "ff")
}

fn hash_until_ff<HashAlg: Digest>(origin_hash: &str, mut hasher: HashAlg) -> String {
    let mut result = String::new();
    let mut current_hash: String = origin_hash.to_string();
    loop {
        hasher.input_str(&current_hash);
        let hash = hasher.result_str();
        if contains_ff(&hash) {
            let rest = split_into_hex_bytes(&hash).into_iter().take_while(|hex| hex != "ff").collect::<String>();
            result += &rest;
            break;
        }
        result += &hash;
        current_hash = hash;
        hasher.reset();
    }
    result
}

pub fn compile_hashfuck(program: String) -> String {
    let splitted: Vec<&str> = program.split(":").collect();
    match splitted.len() {
        2 => {
            let algorithm = splitted[0];
            let origin_hash = splitted[1];
            let full_hash = match algorithm {
                "md5" => hash_until_ff(origin_hash, Md5::new()),
                "sha256" => hash_until_ff(origin_hash, Sha256::new()),
                "sha512" => hash_until_ff(origin_hash, Sha512::new()),
                _ => panic!("Hashing algorithm not implemented")
            };

            println!("Original program: {}", program);
            println!("Hash algorithm: {}", algorithm);
            println!("Hash: {}", origin_hash);
            println!("Extended hash sequence: {}", full_hash);

            hex_to_brainfuck(&full_hash)
        }
        _ => panic!("Format is algorithm:hash")
    }
}

#[cfg(test)]
mod tests {
    use hashfuck::hex_to_brainfuck;

    #[test]
    fn test_hex_to_brainfuck() {
        assert_eq!(hex_to_brainfuck("93f74a28b6d648aec2170182353d0f0fc69072ec1581e49a53cc2f1533455106"), "-]+>[[>[+]<+..]][>+..<.+-.].-.<[");
    }
}
