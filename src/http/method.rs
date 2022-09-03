use std::str::FromStr;

pub enum Method {
    GET,
    POST,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Method, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            _ => Err(MethodError)
        }
    }
}

pub struct MethodError;