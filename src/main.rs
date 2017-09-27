extern crate clap;
extern crate rand;
extern crate regex;

use regex::Regex;
use std::io::{Read, Write};
use std::fs::File;

fn main() {
    let args = clap::App::new("reduce-binsrch")
        .about("Binary search regex reducer for preduce")
        .author("Mateusz Naściszewski <matin1111@wp.pl>")
        .arg(clap::Arg::with_name("REGEX").required(true))
        .arg(clap::Arg::with_name("SEED").required(true))
        .arg(clap::Arg::with_name("replace").default_value(""))
        .get_matches();

    let re = args.value_of("REGEX").unwrap();
    let seed = args.value_of_os("SEED").unwrap();
    let rep = args.value_of("replace").unwrap();

    let matcher = Regex::new(re).unwrap_or_else(|e| {
        eprintln!("Invalid regex /{}/: {}", re, e);
        std::process::exit(1);
    });

    let string: String = {
        let mut s = String::new();

        File::open(seed)
            .expect("Couldn't open seed file")
            .read_to_string(&mut s)
            .expect("Couldn't read seed file");

        s
    };

    let matches: Vec<_> = matcher.captures_iter(string.as_str()).collect();

    run(matches.as_ref(), string.as_str(), rep);
}

// Matches must be sorted, and non-overlapping
fn replace_subset(matches: &[regex::Captures], s: &str, r: &str) -> String {
    let mut last = 0;
    let mut out = String::new();

    for c in matches {
        let m = c.get(0).unwrap();
        out += &s[last..m.start()];
        last = m.end();

        if c.len() == 1 {
            out += r;
        } else {
            let mut tmp = String::new();
            c.expand(r, &mut tmp);
            out += &tmp;
        }
    }

    out += &s[last..];

    out
}

fn try_out(matches: &[regex::Captures], s: &str, r: &str) {
    let mut filename = String::new();
    std::io::stdin().read_line(&mut filename).expect(
        "Expected filename on stdin",
    );
    {
        let name = filename.trim();
        if name.is_empty() {
            eprintln!("IPC Error: Empty filename given, exiting");
            std::process::exit(1);
        }
        let mut f = File::create(name).expect("Couldn't create filename");
        let s = replace_subset(matches, s, r);
        f.write_all(s.as_bytes()).expect("Couldn't write reduction");
        f.flush().unwrap();
    }
    println!();
    std::io::stdout().flush().unwrap();
}

fn run(matches: &[regex::Captures], s: &str, r: &str) {
    let len = matches.len();
    if len < 1 {
        return;
    }
    try_out(matches, s, r);
    if len == 1 {
        return;
    }
    let (f, l) = matches.split_at(len / 2);
    if rand::random() {
        run(f, s, r);
        run(l, s, r);
    } else {
        run(l, s, r);
        run(f, s, r);
    }
}
