// use std::error::Error;
// use std::env; //for argv
// use crypto::md5::Md5;
// use crypto::sha2::{Sha256, Sha512};
// use crypto::digest::Digest;

use std::io::{self, Read};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::char;
use std::thread;
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

fn bruteforce(salt: &str, cases :&Vec<String>, h :&str) {
    for case in cases{
        
        let res = crypt(case, salt).ok().unwrap();
        if h == res{
            println!("  [O]We found password! It's \"{}\".", case);
            println!("{}\n{}", h, res);
            return;
        }
    }
    println!("  [X]We can't find password. Maybe you extend password length and retry it.");
}
fn main() {
    let case = case_gen(3);
    let mut username: String;
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
            "*" => continue,
            _ => v[1],
        };
        println!("[*]Username {}: crack start.", username);

        let hashed: Vec<&str> = h.split('$').collect();
        let salt = format!("${}${}", hashed[1], hashed[2]);
        bruteforce(&salt, &case, h);
    }
}