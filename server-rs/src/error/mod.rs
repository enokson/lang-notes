
#[macro_export]
macro_rules! error_msg {
    ($result: expr) => {
        match $result {
            Ok(ok_result) => Ok(ok_result),
            Err(error) => Err(format!("{}::{} {}", file!(), line!(), error.to_string()))
        }
    };
}