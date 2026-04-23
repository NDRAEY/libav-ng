pub type AVCodecResult<T> = Result<T, AVCodecError>;

#[derive(Debug)]
pub enum AVCodecError {
    Eof,
    TryAgain,
    InvalidRequest,
    NoMemory,
    Other(i32)
}