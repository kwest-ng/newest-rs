use std::collections::BTreeMap;
use std::env::args;
use std::fs::metadata;
use std::io::{prelude::*, stdout, ErrorKind};

use chrono::{DateTime, Local, SecondsFormat};

fn inner() -> Option<i32> {
    let mut map: BTreeMap<DateTime<Local>, String> = BTreeMap::default();

    for arg in args().skip(1) {
        let mdata = match metadata(&arg) {
            Ok(stat) => stat,
            Err(e) => {
                eprintln!("stat for {} failed: {}", &arg, e);
                continue;
            }
        };

        let mtime = match mdata.modified() {
            Ok(time) => time,
            Err(e) => {
                eprintln!("mtime for {} failed: {}", &arg, e);
                continue;
            }
        };

        map.insert(mtime.into(), arg);
    }

    let out = stdout();
    let mut writer = out.lock();

    for (time, file) in map.iter().rev() {
        if let Err(e) = writeln!(writer, "{}: {}", time.to_rfc3339_opts(SecondsFormat::Micros, false), file) {
            match e.kind() {
                ErrorKind::BrokenPipe => {break;}
                _ => {
                    eprintln!("print error: {}", e);
                    return Some(1);
                }
            }
        }
    }

    None
}

fn main() {
    std::process::exit(inner().unwrap_or(0));
}