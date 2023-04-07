use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Error {
    /// Debug message associated with error
    pub msg: &'static str,
    pub typ: Type,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.typ, self.msg)
    }
}

impl std::error::Error for Error {
}

/// Type of error encountered
#[derive(Debug, PartialEq)]
pub enum Type {
    /// Token is invalid
    /// For example, the format of the token is not "HEADER.PAYLOAD.SIGNATURE"
    Invalid,
    /// Token has expired
    Expired,
    /// Not Before (nbf) is set and it's too early to use the token
    Early,
    /// Problem with certificate
    Certificate,
    /// Problem with key
    Key,
    /// Could not download key set
    Connection,
    /// Problem with JWT header
    Header,
    /// Problem with JWT payload
    Payload,
    /// Problem with JWT signature
    Signature,
    /// Internal problem (Signals a serious bug or fatal error)
    Internal,
}

pub(crate) fn err(msg: &'static str, typ: Type) -> Error {
    Error { msg, typ }
}

pub(crate) fn err_inv(msg: &'static str) -> Error {
    err(msg, Type::Invalid)
}

pub(crate) fn err_exp(msg: &'static str) -> Error {
    err(msg, Type::Expired)
}

pub(crate) fn err_nbf(msg: &'static str) -> Error {
    err(msg, Type::Early)
}

pub(crate) fn err_cer(msg: &'static str) -> Error {
    err(msg, Type::Certificate)
}

pub(crate) fn err_key(msg: &'static str) -> Error {
    err(msg, Type::Key)
}

pub(crate) fn err_con(msg: &'static str) -> Error {
    err(msg, Type::Connection)
}

pub(crate) fn err_hea(msg: &'static str) -> Error {
    err(msg, Type::Header)
}

pub(crate) fn err_pay(msg: &'static str) -> Error {
    err(msg, Type::Payload)
}

pub(crate) fn err_sig(msg: &'static str) -> Error {
    err(msg, Type::Signature)
}

pub(crate) fn err_int(msg: &'static str) -> Error {
    err(msg, Type::Internal)
}

#[cfg(test)]
mod tests {}
