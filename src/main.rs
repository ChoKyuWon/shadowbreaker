use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use crypto::sha2::{Sha256, Sha512};
use crypto::digest::Digest;
use crypto::md5;

fn bruteforce(salt :&str, value:&str){
    println!("{}, {}", salt, value);
    /*
    match base64::decode(salt){
        Err(why) => panic!("{}", why.description()),
        Ok(_) => (),
    }
    // println!("{}", _salt);
    */
}

fn main() {
    let path = Path::new("shadow");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
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
            "1" => bruteforce(hashed[2], hashed[3]), //MD5
            "5" => bruteforce(hashed[2], hashed[3]), //SHA256
            "6" => bruteforce(hashed[2], hashed[3]), //SHA512
            _ => println!("{}: This algorithm is not supported", v[0]),
        }
    }
}
