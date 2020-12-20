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

fn bruteforce(salt: &str, cases: &Vec<String>, h: &str) {
    let crypt_lamda = |case: &str, salt: &str, h: &str| h == crypt(case, salt).ok().unwrap();
    static mut FLAG: bool = false;
    let res: Vec<_> = cases
        .par_iter()
        .filter_map(|case| unsafe {
            if FLAG {
                return None;
            }
            if crypt_lamda(case, salt, h) {
                FLAG = true;
                Some(case)
            } else {
                None
            }
        })
        .collect();

    for r in res {
        println!("  [O]We found password! It's \"{}\".", r);
        return;
    }
    println!("  [X]We can't find password. Maybe you change password length and retry it.");
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
            "!" => continue,
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
                println!("[*]Finish generate case. Let's start make some hash.");
                let hashed: Vec<&str> = h.split('$').collect();
                let salt = format!("${}${}", hashed[1], hashed[2]);
                bruteforce(&salt, &case, h);
            }
            "n" => continue,
            _ => continue,
        }
    }
}
