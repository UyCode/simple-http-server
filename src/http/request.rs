use super::method::{Method, MethodError};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{self, Utf8Error};
use super::query_string::QueryString;


#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

/* impl<'buf> Request<'buf> {
    pub fn from_byte_array(buf: &[u8]) -> Result<Request, String> {
        unimplemented!();
    }
}
 */


impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buffer: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        // method 1
        /* match str::from_utf8(buf) {
            Ok(request) => {},
            Err(_) => {
                return Err(ParseError::InvalidEncoding);
            }
        } */

        // method 2
        /* match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
            Ok(request) => request,
            Err(e) => return Err(e)
        } */

        // method 3
        let request = str::from_utf8(buffer)?;//.or(Err(ParseError::InvalidEncoding))?;

        // method 1
        /* match get_next_word(request) {
            Some((method, request)) => {},
            None => return Err(ParseError::InvalidRequest),

        } */

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        //method 1
        /* match path.find('?') {
            Some(i) => {
                query_string = Some(&path[i+1..]);
                path = &path[..1];
            }
            None => {}
        } */


        // method 2
        /* let q = path.find('?');
        if q.is_some() {
            let i = q.unwrap();
            query_string = Some(&path[i+1..]);
            path = &path[..1];
        } */

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..1];
        }
        Ok(Request{
            path,
            query_string,
            method
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    

    /* method 1
    let _iter = request.chars();
    loop {
        let item = iter.next();
        match item {
            Some(c) => {},
            None => break
        }
    }
    */
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "invalid request",
            Self::InvalidEncoding => "invalid Encoding",
            Self::InvalidMethod => "invalid Method",
            Self::InvalidProtocol => "invalid Protocol",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}
