extern crate nix;

use std::env;
use std::fs::read_dir;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::result;
use std::io::{self, BufRead, Write};
use std::num::ParseIntError;
use std::fs::File;

use nix::unistd::getppid;

type Result<T> = result::Result<T, Box<io::Error>>;

fn main() {
    let cwd = env::current_dir().unwrap();
    let cflags = env::var("CFLAGS").unwrap_or(String::from("-g"));

    let mut c_files_vec = vec![];
    _find_c_files(&cwd, &mut c_files_vec);

    let mut cout_pathbuf = cwd.clone();
    cout_pathbuf.push("malloc");
    let cout = cout_pathbuf.to_str().unwrap();

    let cfiles = c_files_vec
        .iter()
        .map(PathBuf::as_path)
        .filter_map(Path::to_str)
        .collect::<Vec<&str>>()
        .join(" ");

    let cargo_profile = env::var("PROFILE").unwrap();
    let pppid = _pppid().unwrap();

    _inject_env_var(pppid, "LD_PRELOAD", "");

    let gcc_out = _exec(&format!("gcc {} {} -o {}", cflags, cfiles, cout), None).unwrap();

    if !gcc_out.status.success() {
        panic!(format!(
            "gcc command failed: {:#?}\n",
            String::from_utf8(gcc_out.stderr)
        ));
    }

    _inject_env_var(
        pppid,
        "LD_PRELOAD",
        &vec!["debug", "release"]
            .iter()
            .filter(|x| **x == cargo_profile)
            .map(|x| format!("{}/target/{}/libkonfiscator.so", cwd.to_string_lossy(), x))
            .collect::<Vec<String>>()
            .join(","),
    );
}

fn _exec(command: &str, stdin: Option<&str>) -> Result<Output> {
    let mut cmd = Command::new("sh");

    cmd.arg("-c").arg(command);

    cmd.stdin(Stdio::piped());
    let mut chld = cmd.spawn().unwrap();

    if let Some(contents) = stdin {
        chld.stdin
            .as_mut()
            .unwrap()
            .write_all(contents.as_bytes())
            .unwrap();
    }

    chld.wait_with_output().map_err(Box::new)
}

fn _find_c_files(path: &PathBuf, files: &mut Vec<PathBuf>) {
    if let Ok(paths) = read_dir(path) {
        for p in paths {
            let path = p.unwrap().path();
            if !path.is_dir() {
                match path.extension().and_then(OsStr::to_str) {
                    Some("c") => files.push(path),
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
        .filter_map(|l| match l {
            Ok(x) => {
                if x.contains("PPid:") {
                    return Some(x);
                }
                None
            }
            _ => None,
        })
        .collect::<Vec<String>>()
        .join("");

    match pppid_out.split_whitespace().last() {
        Some(x) => x.parse::<i32>().map_err(io_errorify),
        None => panic!(),
    }
}

fn _inject_env_var(pid: i32, k: &str, v: &str) {
    let gdb_in = format!("attach {}\ncall putenv (\"{}={}\")\ndetach\n", pid, k, v);
    _exec(&format!("gdb"), Some(&gdb_in)).unwrap();
}

fn io_errorify(e: ParseIntError) -> Box<io::Error> {
    Box::new(io::Error::new(io::ErrorKind::Other, e))
}
