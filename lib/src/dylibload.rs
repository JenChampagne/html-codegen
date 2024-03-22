pub use html_codegen_macros::dylibload;
pub use libloading::{library_filename, Library, Symbol};

#[derive(Debug)]
pub enum Error {
    Library(libloading::Error),
    Inner(Box<dyn std::error::Error>),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Library(e) => write!(f, "Dynamic loading error: {e}"),
            Error::Inner(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<libloading::Error> for Error {
    fn from(value: libloading::Error) -> Self {
        Self::Library(value)
    }
}
