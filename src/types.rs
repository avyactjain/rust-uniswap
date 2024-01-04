use tonic::Status;

#[derive(Debug)]
pub enum UniswapApiError {
    TonicStatus(Status),
}

impl From<Status> for UniswapApiError {
    fn from(value: Status) -> Self {
        UniswapApiError::TonicStatus(value)
    }
}
