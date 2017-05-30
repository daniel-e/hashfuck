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
    hasher.input_str(origin_hash);
    let hash = hasher.result_str();
    let mut result = split_into_hex_bytes(&hash).into_iter().take_while(|hex| hex != "ff").collect::<String>();
    if !contains_ff(&hash) {
        hasher.reset();
        result += &hash_until_ff(&hash, hasher);
    }
    result
}

pub fn compile_hashfuck(program: String) -> String {
    let splitted: Vec<_> = program.split(":").collect();
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
    use hashfuck::{hex_to_brainfuck, hash_until_ff};
    use hashfuck::crypto::sha2::Sha256;

    #[test]
    fn test_hex_to_brainfuck() {
        assert_eq!(hex_to_brainfuck("93f74a28b6d648aec2170182353d0f0fc69072ec1581e49a53cc2f1533455106"), "-]+>[[>[+]<+..]][>+..<.+-.].-.<[");
    }

    #[test]
    fn test_hash_until_ff() {
        assert_eq!(hash_until_ff("a", Sha256::new()), "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bbda3811154d59c4267077ddd8bb768fa9b06399c486e1fc00485116b57c9872f5fd56a5ab49378ccbc9e0335da9a038f58cc180d9f20a3390d2b814a73747596459df7cc9f99b15125f3fd0ac32ec0c63428a7989527903465fa04070d1ddf0722da05beff9f8aad4f85cb42bdc618913c35052affda04911fe8caa229ded674487714c42833099aa9293b6bbf776543b6c306911224219b73828271bf9278920892af415ccf19e5f6fbdd51a0ebcd2070bfc13c94a1f0bcce5cf12b4217d974ffce4cacb4db1c48161f4e07427585a0bf98ef867ed584aaf39b22f34432a66c8ef6b921fbff8475de12f12593b817bdd9afefb94d20eca7556a7bdab7b54ad78389acb1da3cc19ae115c89eec398f9f2d2f3d961");
        assert_eq!(hash_until_ff("g", Sha256::new()), "cd0aa9856147b6c5b4");
    }
}
