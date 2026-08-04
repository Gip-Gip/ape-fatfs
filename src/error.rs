use core::fmt::Debug;
pub use embedded_io::ReadExactError;
pub use embedded_io::Error as IoError;
pub use embedded_io::ErrorKind;
pub use embedded_io::ErrorType;

use crate::io::StdErrWrapper;

/// Error enum with all errors that can be returned by functions from this crate
///
/// Generic parameter `T` is a type of external error returned by the user provided storage
#[derive(Debug)]
#[non_exhaustive]
pub enum FatError<T> {
    /// A user provided storage instance returned an error during an input/output operation.
    Io(T),
    /// A read operation cannot be completed because an end of a file has been reached prematurely.
    UnexpectedEof,
    /// A write operation cannot be completed because `Write::write` returned 0.
    WriteZero,
    /// A parameter was incorrect.
    InvalidInput,
    /// A requested file or directory has not been found.
    NotFound,
    /// A file or a directory with the same name already exists.
    AlreadyExists,
    /// An operation cannot be finished because a directory is not empty.
    DirectoryIsNotEmpty,
    /// File system internal structures are corrupted/invalid.
    CorruptedFileSystem,
    /// There is not enough free space on the storage to finish the requested operation.
    NotEnoughSpace,
    /// The provided file name is either too long or empty.
    InvalidFileNameLength,
    /// The provided file name contains an invalid character.
    UnsupportedFileNameCharacter,
}

impl<T: core::error::Error + Debug + 'static> IoError for FatError<T> {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

impl<T: IoError> From<T> for FatError<T> {
    fn from(error: T) -> Self {
        FatError::Io(error)
    }
}

impl<T> From<ReadExactError<FatError<T>>> for FatError<T> {
    fn from(error: ReadExactError<FatError<T>>) -> Self {
        match error {
            ReadExactError::UnexpectedEof => Self::UnexpectedEof,
            ReadExactError::Other(error) => error,
        }
    }
}

impl<T: IoError> From<ReadExactError<T>> for FatError<T> {
    fn from(error: ReadExactError<T>) -> Self {
        match error {
            ReadExactError::UnexpectedEof => Self::UnexpectedEof,
            ReadExactError::Other(error) => error.into(),
        }
    }
}

#[cfg(feature = "std")]
impl From<FatError<StdErrWrapper>> for std::io::Error {
    fn from(error: FatError<StdErrWrapper>) -> Self {
        match error {
            FatError::Io(io_error) => io_error.into(),
            FatError::UnexpectedEof | FatError::NotEnoughSpace => {
                Self::new(std::io::ErrorKind::UnexpectedEof, error)
            }
            FatError::WriteZero => Self::new(std::io::ErrorKind::WriteZero, error),
            FatError::InvalidInput
            | FatError::InvalidFileNameLength
            | FatError::UnsupportedFileNameCharacter
            | FatError::DirectoryIsNotEmpty => Self::new(std::io::ErrorKind::InvalidInput, error),
            FatError::NotFound => Self::new(std::io::ErrorKind::NotFound, error),
            FatError::AlreadyExists => Self::new(std::io::ErrorKind::AlreadyExists, error),
            FatError::CorruptedFileSystem => Self::new(std::io::ErrorKind::InvalidData, error),
        }
    }
}

impl<T: core::fmt::Display> core::fmt::Display for FatError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FatError::Io(io_error) => write!(f, "IO error: {}", io_error),
            FatError::UnexpectedEof => write!(f, "Unexpected end of file"),
            FatError::NotEnoughSpace => write!(f, "Not enough space"),
            FatError::WriteZero => write!(f, "Write zero"),
            FatError::InvalidInput => write!(f, "Invalid input"),
            FatError::InvalidFileNameLength => write!(f, "Invalid file name length"),
            FatError::UnsupportedFileNameCharacter => write!(f, "Unsupported file name character"),
            FatError::DirectoryIsNotEmpty => write!(f, "Directory is not empty"),
            FatError::NotFound => write!(f, "No such file or directory"),
            FatError::AlreadyExists => write!(f, "File or directory already exists"),
            FatError::CorruptedFileSystem => write!(f, "Corrupted file system"),
        }
    }
}

impl<T: core::error::Error + 'static> core::error::Error for FatError<T> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        if let FatError::Io(io_error) = self {
            Some(io_error)
        } else {
            None
        }
    }
}

#[cfg(feature = "std")]
impl core::fmt::Display for StdErrWrapper {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "pls implement")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for StdErrWrapper {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

// /// Trait that should be implemented by errors returned from the user supplied storage.
// ///
// /// Implementations for `std::io::Error` and `()` are provided by this crate.
// pub trait IoError: core::fmt::Debug {
//     fn is_interrupted(&self) -> bool;
//     fn new_unexpected_eof_error() -> Self;
//     fn new_write_zero_error() -> Self;
// }
//
// impl<T: core::fmt::Debug + IoError> IoError for Error<T> {
//     fn is_interrupted(&self) -> bool {
//         match self {
//             Error::<T>::Io(io_error) => io_error.is_interrupted(),
//             _ => false,
//         }
//     }
//
//     fn new_unexpected_eof_error() -> Self {
//         Error::<T>::UnexpectedEof
//     }
//
//     fn new_write_zero_error() -> Self {
//         Error::<T>::WriteZero
//     }
// }
//
// impl IoError for () {
//     fn is_interrupted(&self) -> bool {
//         false
//     }
//
//     fn new_unexpected_eof_error() -> Self {
//         // empty
//     }
//
//     fn new_write_zero_error() -> Self {
//         // empty
//     }
// }
//
// #[cfg(feature = "std")]
// impl IoError for std::io::Error {
//     fn is_interrupted(&self) -> bool {
//         self.kind() == std::io::ErrorKind::Interrupted
//     }
//
//     fn new_unexpected_eof_error() -> Self {
//         Self::new(std::io::ErrorKind::UnexpectedEof, "failed to fill whole buffer")
//     }
//
//     fn new_write_zero_error() -> Self {
//         Self::new(std::io::ErrorKind::WriteZero, "failed to write whole buffer")
//     }
// }
