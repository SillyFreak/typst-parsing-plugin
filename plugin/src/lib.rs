#[cfg(target_arch = "wasm32")]
use wasm_minimal_protocol::wasm_func;

pub mod ast;
pub mod parser;

fn cbor_encode<T>(value: &T) -> Result<Vec<u8>, ciborium::ser::Error<std::io::Error>>
where
    T: serde::Serialize + ?Sized,
{
    let mut writer = Vec::new();
    ciborium::into_writer(value, &mut writer)?;
    Ok(writer)
}

trait MapErrToString<T> {
    fn map_err_to_string(self) -> Result<T, String>;
}

impl<T, E: ToString> MapErrToString<T> for Result<T, E> {
    fn map_err_to_string(self) -> Result<T, String> {
        self.map_err(|err| err.to_string())
    }
}

#[cfg(target_arch = "wasm32")]
wasm_minimal_protocol::initiate_protocol!();

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn parse(expr: &[u8]) -> Result<Vec<u8>, String> {
    let expr: String = ciborium::from_reader(expr).map_err_to_string()?;
    let expr = parser::parse(&expr).map_err_to_string()?;
    let expr = cbor_encode(&expr).map_err_to_string()?;
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        parse(&cbor_encode("(foo + 1)").unwrap());
    }
}