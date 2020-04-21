use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

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
        for hs in hashed{
            println!("{} : {}", v[0], hs);
        }
    }
}
