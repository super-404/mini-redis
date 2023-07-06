use thiserror::Error;
#[derive(Error,Debug)]
pub enum Error{
    #[error("read file failed")]
    ReadFileFailed,
    #[error("open file failed")]
    OpenFileFailed,
    #[error("write file failed")]
    WriteFileFailed,
    #[error("sync data failed")]
    SyncDataFailed
}