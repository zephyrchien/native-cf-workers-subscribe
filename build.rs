use std::env;
use std::io::{BufRead, BufReader};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn main() {
    let conf_path = Path::new("deploy.conf");
    let src_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut out_path = PathBuf::from_str(&src_path).unwrap();
    out_path.push("src");
    out_path.push("generated");
    if fs::read_dir(&out_path).is_err() {
        fs::create_dir(&out_path).unwrap();
    }
    out_path.push("config.rs");

    let mut code = Vec::<String>::with_capacity(8);
    let reader = BufReader::new(File::open(&conf_path).unwrap());
    for line in reader.lines() {
        let line = line.as_ref().unwrap().trim();
        let (key, val) = split_kv(line);
        code.push(format!(r#"pub const {}: &str = "{}";"#, key, val));
    }

    fs::write(&out_path, code.join("\n")).unwrap();

    println!("cargo:rerun-if-changed={}", out_path.to_str().unwrap());
}

fn split_kv(s: &str) -> (&str, &str) {
    let mut it = s.split('=');
    let key = it.next().unwrap().trim();
    let val = it.next().unwrap().trim();
    (key, val)
}
