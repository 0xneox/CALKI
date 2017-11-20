//-32003             Query class error
//-32006             Transaction authentication error
//-32099             Request timed out
pub enum ErrorCode {
    QueryError,
    TxAuthError,
    TimeOut,
}

impl ErrorCode {
    pub fn query_error() -> i64 {
        -32003
    }

    pub fn tx_auth_error() -> i64 {
        -32006
    }

    pub fn time_out_error() -> i64 {
        -32099
    }
}
