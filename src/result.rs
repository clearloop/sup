//! Sup Result
use etc::Error as Etc;
use std::{
    error::Error as ErrorTrait,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as Io,
    result::Result as StdResult,
};
use toml::{de::Error as DeToml, ser::Error as SerToml};

/// The custom sup error
pub struct Sup(String);
impl Display for Sup {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.0)
    }
}

/// Error generator
macro_rules! error {
    ($($e:ident),*) => {
        /// Sup Error
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            $($e(String),)+
        }

        impl Display for Error {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                match self {
                    $(Error::$e(e) => e.fmt(f),)+
                }
            }
        }

        impl ErrorTrait for Error {}

        $(
            impl From<$e> for Error {
                fn from(e: $e) -> Error {
                    Error::$e(format!("{}", e))
                }
            }
        )+
    };
}

error! {Etc, Io, Sup, DeToml, SerToml}

/// Sup Result
pub type Result<T> = StdResult<T, Error>;
