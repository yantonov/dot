pub fn to_result<T>(opt: Option<T>, message: &str) -> Result<T, String> {
    match opt {
        None => Err(message.to_string()),
        Some(v) => Ok(v)
    }
}