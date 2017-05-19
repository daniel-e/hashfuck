extern crate argparse;
extern crate crypto;

#[cfg(feature = "interpreter")]
extern crate brainfuck;

use argparse::{ArgumentParser, Store};

#[cfg(feature = "interpreter")]
use argparse::StoreTrue;

use std::collections::HashMap;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

fn split_into_hex_bytes(s: &str) -> Vec<String> {
    let foo = s.chars().collect::<Vec<_>>();
    let bar = foo.chunks(2).collect::<Vec<_>>();
    let baz = bar.iter().map(|&x| x.iter().collect::<String>()).collect::<Vec<_>>();
    baz
}

fn hex_to_brainfuck(hash: &str) -> String {
    let bf_ops: HashMap<i32, &str> = [
        (0, ">"), (1, "<"), (2, "+"), (3, "-"), (4, "."), (5, "."), (6, "["), (7, "]"),
    ].iter().cloned().collect();

    let baz = split_into_hex_bytes(hash);
    let ops = baz.iter().map(|hex| i32::from_str_radix(hex, 16)).map(|dec| dec.unwrap() % 8).collect::<Vec<_>>();
    let bf = ops.iter().map(|op| *bf_ops.get(&op.clone()).unwrap()).collect::<String>();
    bf
}

fn contains_ff(s: &str) -> bool {
    split_into_hex_bytes(&s).into_iter().any(|x| x == "ff")
}

fn sha256_rotate_until_ff(origin_hash: &str) -> String {
    let mut hasher = Sha256::new();
    let mut result = String::new();
    let mut current_hash: String = origin_hash.to_string();

    loop {
        hasher.input_str(&current_hash);
        let hash = hasher.result_str();
        hasher.reset();
        println!("hash: {:?}", &hash);
        if contains_ff(&hash) {
            let rest = split_into_hex_bytes(&hash).into_iter().take_while(|hex| hex != "ff").collect::<String>();
            println!("rest: {:?}", &rest);
            result += &rest;
            break;
        } else {
            result += &hash;
        }
        current_hash = hash;
    }
    result
}

#[cfg(feature = "interpreter")]
fn interpret(bf: String) {
    brainfuck::eval_string(&bf).expect("Interpreter error");
}

fn main() {
    let mut program = String::new();
    #[cfg(feature = "interpreter")]
    let mut option_interpret = false;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Hashfuck Interpreter");
        ap.refer(&mut program).add_argument("program", Store, "Hashfuck program code");
        #[cfg(feature = "interpreter")]
        ap.refer(&mut option_interpret).add_option(&["-i", "--interpret"], StoreTrue, "Run BF interpreter");
        ap.parse_args_or_exit();
    }

    if !program.is_empty() {
        let splitted: Vec<&str> = program.split(":").collect::<Vec<_>>();
        let algorithm = splitted[0];
        let origin_hash = splitted[1];

        let full_hash = match algorithm {
            "sha256" => sha256_rotate_until_ff(origin_hash),
            _ => panic!("Not implemented")
        };

        println!("Original program: {}", program);
        println!("Hash algorithm: {}", algorithm);
        println!("Hash: {}", origin_hash);

        println!("Full hash: {}", full_hash);

        let bf = hex_to_brainfuck(&full_hash);
        println!("Brainfuck: {}", bf);

        #[cfg(feature = "interpreter")]
        interpret(bf);
    }
}