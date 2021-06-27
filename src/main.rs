use clap::{App, Arg};
use bit_vec::BitVec;
use std::fs::File;
use std::io::{Read, Write};

fn enc(a: bool, b: bool) -> (bool, bool) {
    match (a, b) {
        (false, false) => (false, true),
        (false, true) => (true, false),
        (true, false) => (true, true),
        (true, true) => (false, false),
    }
}

fn dec(a: bool, b: bool) -> (bool, bool) {
    match (a, b) {
        (false, true) => (false, false),
        (true, false) => (false, true),
        (true, true) => (true, false),
        (false, false) => (true, true),
    }
}

fn encode_array(first: BitVec, second: BitVec) -> (BitVec, BitVec) { //len a = len b
    let mut res1 = BitVec::new();
    let mut res2 = BitVec::new();
    for (i, j) in first.iter().zip(second.iter()) {
        let (new1, new2) = enc(i, j);
        res1.push(new1);
        res2.push(new2);
    }
    (res1, res2)
}

fn decode_array(first: BitVec, second: BitVec) -> (BitVec, BitVec) { //len a = len b
    let mut res1 = BitVec::new();
    let mut res2 = BitVec::new();
    for (i, j) in first.iter().zip(second.iter()) {
        let (new1, new2) = dec(i, j);
        res1.push(new1);
        res2.push(new2);
    }
    (res1, res2)
}

fn main() {
    let matches = App::new("krestrage")
        .subcommand(App::new("encode")
            .arg(Arg::with_name("file")))
        .subcommand(App::new("decode")
            .args(&[Arg::with_name("file1"), Arg::with_name("file2")]))
        .get_matches();
    if let Some(encode_matches) = matches.subcommand_matches("encode") {
        if let Some(filename) = encode_matches.value_of("file") {
            let mut file = File::open(filename).expect("No file");
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).unwrap();
            let len = bytes.len();
            println!("len {}", len);
            if len % 2 != 0 {
                panic!("Nu vot hz che s etim delat")
            }
            let half1 = BitVec::from_bytes(&bytes[..len / 2]);
            println!("len h1 {}", &bytes[..len / 2].len());
            let half2 = BitVec::from_bytes(&bytes[len / 2..]);
            println!("len h2 {}", &bytes[len / 2..].len());
            let (new1, new2) = encode_array(half1, half2);
            let new1 = &*new1.to_bytes();
            let new2 = &*new2.to_bytes();
            let mut file1 = File::create(filename.to_owned() + ".enc1").expect("lol");
            let mut file2 = File::create(filename.to_owned() + ".enc2").expect("lol");
            file1.write_all(new1).unwrap();
            file2.write_all(new2).unwrap();
            println!("Succes")
        } else {
            eprintln!("no file")
        }
    } else if let Some(decode_matches) = matches.subcommand_matches("decode") {
        match (decode_matches.value_of("file1"), decode_matches.value_of("file2")) {
            (Some(filename1), Some(filename2)) => {
                let mut file1 = File::open(filename1).expect("lol");
                let mut file2 = File::open(filename2).expect("lol");
                let mut bytes1 = Vec::new();
                let mut bytes2 = Vec::new();
                file1.read_to_end(&mut bytes1).unwrap();
                file2.read_to_end(&mut bytes2).unwrap();
                if bytes1.len() != bytes2.len() {
                    panic!("Nu vot hz che s etim delat")
                }
                let (res1, res2) =
                    decode_array(BitVec::from_bytes(&bytes1),
                                 BitVec::from_bytes(&bytes2));
                let (new1, new2) = (
                    &*res1.to_bytes(),
                    &*res2.to_bytes()
                    );
                let mut file_res = File::create("decoded").expect("lol");
                file_res.write(new1);
                file_res.write(new2);
            }
            _ => { eprintln!("no files!") }
        }
    } else {
        eprintln!("Usage: encode [file]\n       decode [file.enc1] [file.enc2] ")
    }
}
