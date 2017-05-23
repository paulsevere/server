#![allow(unused_variables)]
use std::thread;
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use std::fs::{self, DirEntry};

use std::io::prelude::*;
use std::io;




fn main() {
    let path = Path::new("/Users/patrick/Projects");

    visit_dirs(&path, &printDir);
    // let (tx, rx) = channel();
    // let tx = createThread(tx);
    // let tx = createThread(tx);
    // read_user_input();
    // println!("{}", rx.recv().unwrap());
    // println!("{}", rx.recv().unwrap());

}


fn read_user_input() {
    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();
}


fn createThread(s: Sender<u8>) -> Sender<u8> {
    let _s = s.clone();
    thread::spawn(move || { _s.send(1); });
    s
}

fn printDir(dir: &DirEntry) -> fn fn()-> {
    println!("{:?}", dir);
}

fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
