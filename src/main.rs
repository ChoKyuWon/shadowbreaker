use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::char;
// use std::error::Error;
// use std::env; //for argv
// use crypto::sha2::{Sha256, Sha512};
// use crypto::digest::Digest;
// use crypto::md5;

fn case_gen(len: usize) -> Vec<String>{
    let mut v : Vec<String> = Vec::new();
    const start : char = 32 as char;
    const end : char = 126 as char;
    if(len == 1){
        for ch in 32..126{
            v.push(String::from(ch as u8 as char));
        }
        return v;
    }
    for index in 0..len{
        let prev: String;
        for ch in 32..126{
            v.push("aa".to_string());
        }
    }
    return v;

    //v.push(std::iter::repeat(start).take(len).collect::<String>());
}

fn md5_bruteforce(salt :&str, value:&str){
    println!("{}, {}", salt, value);
}

fn sha256_bruteforce(salt :&str, value:&str){
    println!("{}, {}", salt, value);
}

fn sha512_bruteforce(salt :&str, value:&str){
    println!("{}, {}", salt, value);
}

fn main() {
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
        let h = match v[1] {
            "!!" => continue,
            _ => v[1],
        };
        let hashed: Vec<&str> = h.split('$').collect();
        match hashed[1]{
            "1" => md5_bruteforce(hashed[2], hashed[3]), //MD5
            "5" => sha256_bruteforce(hashed[2], hashed[3]), //SHA256
            "6" => sha512_bruteforce(hashed[2], hashed[3]), //SHA512
            _ => println!("{}: This algorithm is not supported", v[0]),
        }
    }
}