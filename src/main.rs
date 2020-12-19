use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::char;

// use std::error::Error;
// use std::env; //for argv
use crypto::md5::Md5;
use crypto::sha2::{Sha256, Sha512};
use crypto::digest::Digest;

use pwhash::unix::crypt;

fn case_gen(len: usize) -> Vec<String>{
    let mut v : Vec<String> = Vec::new();
    if(len == 1){
        for ch in 32..127{
            v.push(String::from(ch as u8 as char));
        }
        return v;
    }
    let prevs = case_gen(len - 1);
    for prev in prevs{
        for ch in 32..127{
            let mut tmp_string:String = prev.to_owned();
            tmp_string.push_str(&String::from(ch as u8 as char));
            v.push(tmp_string);
        }
    }
    return v;

    //v.push(std::iter::repeat(start).take(len).collect::<String>());
}

// fn md5_bruteforce(salt :&str, value:&str, case:&Vec<String>){
//     let hashed_case = Vec::new();
//     println!("{}, {}", salt, value);
// }

// fn sha256_bruteforce(salt :&str, value:&str, case:&Vec<String>){
//     println!("{}, {}", salt, value);
// }

// fn sha512_bruteforce(salt :&str, value:&str, case:&Vec<String>){
//     println!("{}, {}", salt, value);
// }

fn __bruteforce(func :&str, salt :&str, value:&str, cases :&Vec<String>){
    let hashed_case: Vec<String> = Vec::new();
    match func{
        "1" => {
            for case in cases{
                let mut h = Md5::new();
                h.input(case.as_bytes());
                let mut out = [0; 16];
                h.result(&mut out);
                println!("Hashed value: {:?}, origin value: {}",out, case);
            }
        },
        "5" => {
            for case in cases{
                let mut h = Sha256::new();
                h.input(case.as_bytes());
                let mut out = [0; 32];
                h.result(&mut out);
                println!("Hashed value: {:?}, origin value: {}",out, case);
            }
        },
        "6" => {
            for case in cases{
                let mut h = Sha512::new();
                h.input(case.as_bytes());
                let mut out = [0; 64];
                h.result(&mut out);
                println!("Hashed value: {:?}, origin value: {}",out, case);
            }
        },
        _ => println!("{}: This algorithm is not supported, sorry.", func),
    }
}

fn bruteforce(salt: &str, cases :&Vec<String>){
    for case in cases{
        println!("{}", crypt(case, salt).ok().unwrap());
    }
}
fn main() {
    let case = case_gen(1);
    let path = Path::new("shadow");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.to_string()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.to_string()),
        Ok(_) => (),
    }
    let shadows: Vec<&str> = s.split('\n').collect();
    for shadow in shadows{
        let v: Vec<&str> = shadow.split(':').collect();
        let username = v[0];
        let h = match v[1] {
            "!!" => continue,
            _ => v[1],
        };
        println!("[*]Username {}: crack start.", username);

        bruteforce(h, &case);

        // let hashed: Vec<&str> = h.split('$').collect();
        // __bruteforce(hashed[1], hashed[2], hashed[3], &case);
    }
}