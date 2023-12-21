use casper_types::ApiError;

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum NexfiDropError {
    AlreadyInitialized = 10001,
    InvalidArgument = 10002,
    MissingKey = 10003,
    CantReadUref = 10004,
    CantReadBalances = 10005
}

impl From<NexfiDropError> for ApiError {
    fn from(error: NexfiDropError) -> Self {
        ApiError::User(error as u16)
    }
}