mod balances;
mod system;

pub type RequestResult<T> = Result<Option<T>, subxt::Error>;