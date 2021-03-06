use std::{
    fs, path,
    process::Stdio,
    sync::{Arc, Mutex},
};

use chrono::{Datelike, Timelike};
use once_cell::sync::Lazy;
use tokio::{
    io::AsyncReadExt,
    process::{Child, Command},
};
use std::io::Write;

pub mod commands;
pub mod loops;

#[derive(Clone)]
pub struct EnviromentState {
    pub command_queue: Arc<Mutex<deadqueue::unlimited::Queue<String>>>,
    pub runstate: Arc<Mutex<Runstate>>,
    pub server_process: Arc<Mutex<Child>>,
}

#[derive(PartialEq, Debug)]
pub enum Runstate {
    Running,
    Closing,
}

pub fn spawn_server() -> tokio::process::Child {
    let mut child;
    #[cfg(target_os = "windows")]
    {
        child = Command::new("C:\\Program Files\\nodejs\\npx.exe")
            .arg("ts-node")
            .arg("serverenv\\server\\server.ts")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
    }

    #[cfg(target_os = "linux")]
    {
        child = Command::new("npx")
            .arg("ts-node")
            .arg("serverenv/server/server.ts")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
    }
    printout("server process started");
    let mut child_stdout = child.stdout.take().unwrap();
    let mut child_stderr = child.stderr.take().unwrap();

    tokio::spawn(async move {
        let mut line = String::new();
        loop {
            let byte;
            tokio::select! {
                res = child_stdout.read_u8() => {
                    match res {
                        Ok(res_byte) => byte = res_byte,
                        Err(_) => {
                            printout("server process piping stopped");
                            break;
                        }
                    }
                }
                res = child_stderr.read_u8() => {
                    match res {
                        Ok(res_byte) => byte = res_byte,
                        Err(_) => {
                            printout("server process piping stopped");
                            break;
                        }
                    }
                }
            }
            let char = char::from_u32(byte as u32).unwrap();

            if char == '\n' {
                if line.chars().count() == 0 {
                    //ignore empty lines
                    continue;
                }

                printout(format!("{}", line));
                line = String::new();
                continue;
            } else if char == '\r' || char == '^' {
                //do nothing
            } else {
                line = format!("{}{}", line, &char);
            }
        }
    });

    child
}


pub fn printout(text: impl std::fmt::Display) {
    let time = chrono::prelude::Local::now();
    let time = format!(
        "{}-{}-{} {}:{}:{}",
        time.year(),
        time.month(),
        time.day(),
        time.hour(),
        time.minute(),
        time.second()
    );
    let text = format!("{} || {}", text, time);
    println!("{}", text);

    if !path::Path::new("./serverenv/logs.txt").exists() {
        fs::File::create("./serverenv/logs.txt").unwrap();
    }
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("./serverenv/logs.txt")
        .unwrap();

    writeln!(file,"{}", text).unwrap();
}

pub static REPEAT_ON_EXIT: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
