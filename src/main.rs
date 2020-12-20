// use std::error::Error;
// use std::env; //for argv
// use crypto::md5::Md5;
// use crypto::sha2::{Sha256, Sha512};
// use crypto::digest::Digest;

use pwhash::unix::crypt;
use rayon::prelude::*;
use std::char;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

fn case_gen(len: usize) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    if (len == 1) {
        for ch in 32u8..127 {
            v.push(String::from(ch as char));
        }
        return v;
    }
    let prevs = case_gen(len - 1);
    for prev in prevs {
        for ch in 32u8..127 {
            let mut tmp_string: String = prev.to_owned();
            tmp_string.push_str(&String::from(ch as char));
            v.push(tmp_string);
        }
    }
    return v;
}

// fn __bruteforce(func :&str, salt :&str, value:&str, cases :&Vec<String>){
//     let hashed_case: Vec<String> = Vec::new();
//     match func{
//         "1" => {
//             for case in cases{
//                 let mut h = Md5::new();
//                 h.input(case.as_bytes());
//                 let mut out = [0; 16];
//                 h.result(&mut out);
//                 println!("Hashed value: {:?}, origin value: {}",out, case);
//             }
//         },
//         "5" => {
//             for case in cases{
//                 let mut h = Sha256::new();
//                 h.input(case.as_bytes());
//                 let mut out = [0; 32];
//                 h.result(&mut out);
//                 println!("Hashed value: {:?}, origin value: {}",out, case);
//             }
//         },
//         "6" => {
//             for case in cases{
//                 let mut h = Sha512::new();
//                 h.input(case.as_bytes());
//                 let mut out = [0; 64];
//                 h.result(&mut out);
//                 println!("Hashed value: {:?}, origin value: {}",out, case);
//             }
//         },
//         _ => println!("{}: This algorithm is not supported, sorry.", func),
//     }
// }

// fn crypt_lamda(case: &str, salt: &str, ret: &mut String) {
//     *ret = crypt(case, salt).ok().unwrap();
// }

fn bruteforce(salt: &str, cases: &Vec<String>, h: &str) {
    // cases
    //     .par_iter()
    //     .for_each(|case| crypt(case, salt).ok().unwrap());

    // let crypt_lamda = |case: &str, salt: &str| crypt(case, salt).ok().unwrap();
    for case in cases.iter() {
        let res = crypt(case, salt).ok().unwrap();
        if h == res {
            println!("  [O]We found password! It's \"{}\".", case);
            println!("{}\n{}", h, res);
            return;
        } else {
            let p = format!("{}\n", case);
            io::stderr()
                .write_all(p.as_bytes())
                .expect("Write Error...");
        }
    }
    println!("  [X]We can't find password. Maybe you extend password length and retry it.");
}

fn main() {
    let path = Path::new("shadow");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.to_string()),
        Ok(_) => (),
    }
    let shadows: Vec<&str> = s.split('\n').collect();
    for shadow in shadows {
        let v: Vec<&str> = shadow.split(':').collect();
        let username = v[0];
        let h = match v[1] {
            "!!" => continue,
            "*" => continue,
            _ => v[1],
        };
        let mut buffer = String::new();
        let stdin = io::stdin();
        print!("[*]Username {}: start crack? (y/n):", username);
        io::stdout().flush().unwrap();
        stdin.read_line(&mut buffer).expect("Some Err");

        match &(buffer.trim())[..] {
            "y" => {
                let l: usize;
                loop {
                    print!("Input password length:");
                    io::stdout().flush().unwrap();
                    buffer = String::new();
                    stdin.read_line(&mut buffer).expect("Some Err");

                    l = match buffer.trim().parse::<usize>() {
                        Ok(len) => len,
                        Err(err) => {
                            println!("Buffer:{}, {}", buffer, err);
                            println!("Please input positive integer!");
                            continue;
                        }
                    };
                    break;
                }
                let case = case_gen(l);
                let hashed: Vec<&str> = h.split('$').collect();
                let salt = format!("${}${}", hashed[1], hashed[2]);
                bruteforce(&salt, &case, h);
            }
            "n" => continue,
            _ => continue,
        }
    }
}
