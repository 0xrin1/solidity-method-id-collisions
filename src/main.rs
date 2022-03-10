use ethers::{core::types::H256, utils::keccak256};
use itertools::Itertools;

fn main() {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let rev_alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let mut i = 1;
    let mut ii = 1;

    println!("finding collision...");

    'outer: while i < 26 {
        while ii < 26 {
            for perm in alphabet.iter().permutations(i).unique() {
                for rev_perm in rev_alphabet.iter().permutations(ii).unique() {
                    let method_name: String = gen_method_name(perm.clone());
                    let method_id: String = gen_method_id(&method_name);
                    let method_name_1: String = gen_method_name(rev_perm.clone());
                    let method_id_1: String = gen_method_id(&method_name_1);

                    if method_id == method_id_1 && method_name != method_name_1 {
                        println!("method_name: {} {}", method_name, method_id);
                        println!("method_name_1: {} {}", method_name_1, method_id_1);

                        break 'outer;
                    }
                }
            }

            ii += 1;
        }

        i += 1;
        ii = 1;
    }
}

fn gen_method_name(name_vec: Vec<&char>) -> String {
    let string: String = name_vec.clone().into_iter().collect();
    return format!("{}{}", string, "()");
}

fn gen_method_id(name: &String) -> String {
    let method_hash: H256 = keccak256(name).into();
    let method_hash_string: String = format!("{:#x}", method_hash);
    return (&method_hash_string[..10]).to_string();
}
