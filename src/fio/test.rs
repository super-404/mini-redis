use std::{path::PathBuf, io::Read};

use log::{debug, error, log_enabled, info, Level};
use crate::fio::file_io::{FileIO, IOManager};
#[test]
fn test_new (){
    env_logger::init();
    info!("starting up");
    let p = PathBuf::from("hello.txt");
    let r = FileIO::new(p);
    println!("{:?}",r);
}
#[test]
fn test_read(){
    env_logger::init();
    info!("starting up");
    let p = PathBuf::from("hello.txt");
    let f = FileIO::new(p).unwrap();
    //f.unwrap_or();
    let mut buf = [0 ;10];
    let res = f.read(&mut buf, 20);

    println!("{:?}",res);
    println!("{:?}",String::from_utf8(buf.to_vec()));
}

#[test]
fn test_write(){
    env_logger::init();
    info!("starting up");
    let p = PathBuf::from("hello.txt");
    let f = FileIO::new(p).unwrap();
    //f.unwrap_or();
    //let mut buf = [0 ;10];
    let res = f.write("heee".as_bytes());

    println!("{:?}",res);
   // println!("{:?}",String::from_utf8(buf.to_vec()));
}