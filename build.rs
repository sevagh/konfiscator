extern crate nix;

use std::env;
use std::fs::read_dir;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::result;
use std::io::{self, BufRead};
use nix::unistd::getppid;
use std::fs::File;

type Result<T> = result::Result<T, Box<io::Error>>;

fn main() {
    let mut cwd = env::current_dir().expect("Couldn't get $CWD");
    let cflags = env::var("CFLAGS").unwrap_or_default();

    let mut c_files_vec = vec![];
    _find_c_files(&cwd, &mut c_files_vec);

    cwd.push("malloc");
    let cout = cwd.to_str().expect("Couldn't convert out path to str");

    let cfiles = c_files_vec.iter().map(PathBuf::as_path).filter_map(Path::to_str).collect::<Vec<&str>>().join(" ");

    _exec(&format!("gcc {} {} -o {}", cflags, cfiles, cout)).expect("Error invoking gcc");

    _pppid();
}

fn _exec(cmd: &str) -> Result<Output> {
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .map_err(Box::new)
}

fn _find_c_files(path: &PathBuf, files: &mut Vec<PathBuf>) {
    if let Ok(paths) = read_dir(path) {
        for p in paths {
            let path = p.expect("error iterating through paths").path();
            if !path.is_dir() {
                match path.extension().and_then(OsStr::to_str) {
                    Some("c") => {
                        files.push(path)
                    },
                    _ => continue,
                }
            }
        }
    }
}

fn _pppid() -> Result<i32> {
    let ppid = getppid();

    let file = File::open(&format!("/proc/{}/status", ppid))?;
    let buf = io::BufReader::new(file);

    let pppid_out = buf.lines()
        .filter_map(|l| {
            match l {
                Ok(x) => {
                    if x.contains("PPid:") {
                        return Some(x)
                    }
                    None
                }
                _ => None
            }
        })
        .collect::<Vec<String>>().join("");

    println!("My pppid is: {}", pppid_out);
    Ok(0)
}
