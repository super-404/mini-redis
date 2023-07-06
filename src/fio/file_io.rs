use crate::errors::Error;
use log::{debug, error, info, log_enabled, Level};
use parking_lot::RwLock;
use std::fs::File;
use std::io::Write;
use std::{
    os::windows::prelude::FileExt,
    path::{Path, PathBuf},
    sync::Arc,
};
pub type Result<T> = std::result::Result<T, Error>;
pub trait IOManager: Send + Sync {
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize>;
    fn write(&self,buf:&[u8])->Result<usize>;
    fn sync(&self)->Result<()>;
}
#[derive(Debug)]
pub struct FileIO {
    file: Arc<RwLock<File>>,
}

impl FileIO {
    pub fn new(file_name: PathBuf)-> Result<FileIO>{
        match File::options()
            .read(true)
            .append(true)
            .open(file_name) {
                Ok(f)=> Ok(FileIO{file:Arc::new(RwLock::new(f))}),
                Err(e)=>{
                    let s = e.to_string();
                    error!("open file failed: {s}");
                    Err(Error::OpenFileFailed)
                }
         }
    }
}
impl IOManager for FileIO {
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize> {
        let result;
        //获取读锁
        {
            let r_lock = self.file.read();
            result = r_lock.seek_read(buf, offset);
        }
        //读锁释放
        match result {
            Ok(u) => Ok(u),
            Err(e) => {
                let s = e.to_string();
                //error!("open file failed: {s}");
                error!("fail to read file");
                Err(Error::ReadFileFailed)
            }
        }
    }
    fn write(&self,buf:&[u8])->Result<usize> {
        let res;
        //获取写锁
        {
            let mut w_lock = self.file.write();
            res = w_lock.write(buf);
        }
        match res {
            Ok(u) => Ok(u),
            Err(e)=>{
                error!("write file failed");
                Err(Error::WriteFileFailed)
            }
        }
        //写锁释放
    }   
    fn sync(&self)->Result<()> {
        let r_lock = self.file.read();
        if let Err(e)  = r_lock.sync_all(){
            error!("sync data failed");
            return Err(Error::SyncDataFailed);
        }
        Ok(())
    }
}
