

use std::{fs::read_to_string, error};

 

fn render() -> Result<String, MyError> {
  let file = std::env::var("MARKDOWN")?;
  let source = read_to_string(file)?;
  Ok(source)
}

#[derive(thiserror::Error, Debug)]
enum MyError {
  #[error("Environment variable not found")]
  EnvironmentVariableNotFound(#[from] std::env::VarError),
  #[error(transparent)]
  IOError(#[from] std::io::Error),
}
// #[derive(thiserror::Error,Display,Debug,)]
// pub enum Error {
//     #[error("first letter must be lowercase but was {:?}", first_char(.0))]
//     WrongCase(String),
//     // #[error("invalid index {idx}, expected at least {} and at most {}", .limits.lo, .limits.hi)]
//     // OutOfBounds { idx: usize, limits: Limits },
// }
// fn first_char(char:&String)->char{
//     return char.as_bytes().to_vec()[0] as char;
// }
// fn appear_errors()->Result<String,Error>{
    
//     let s =String::from("Value");
//     if s.is_ascii() {
//         Err(Error::WrongCase(s))
//     }   else {
//         Ok(s)
//     }

#[test]
fn test_this_error(){

    let html = render();
    println!("{:?}",html);
    println!("{}", html.err().unwrap());
    
    
}

use log::{debug, error, log_enabled, info, Level};
#[test]
fn test_env_logger(){
  env_logger::init();
  error!("hhhh");
  info!("starting up");
}
