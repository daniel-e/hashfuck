extern crate crypto;

use std::collections::HashMap;
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
    let bf_ops: HashMap<i32, &str> = [
        (0, ">"), (1, "<"), (2, "+"), (3, "-"), (4, "."), (5, "."), (6, "["), (7, "]"),
    ].iter().cloned().collect();

    let hex = split_into_hex_bytes(hash);
    let ops = hex.iter().map(|hex| i32::from_str_radix(hex, 16)).map(|dec| dec.unwrap() % 8).collect::<Vec<_>>();
    ops.iter().map(|op| *bf_ops.get(&op.clone()).unwrap()).collect::<String>()
}

fn contains_ff(s: &str) -> bool {
    split_into_hex_bytes(&s).into_iter().any(|x| x == "ff")
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
    let splitted: Vec<&str> = program.split(":").collect::<Vec<_>>();
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
