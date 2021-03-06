use std::io;
use ini;
use failure::Backtrace;
use block_modes::BlockModeError;
use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Error reading keys file: {}", _0)]
    Io(io::Error, Backtrace),
    #[fail(display = "Decryption failed")]
    BlockMode(BlockModeError, Backtrace),
    #[fail(display = "Error parsing the INI file: {}", _0)]
    Ini(ini::ini::Error, Backtrace),
    #[fail(display = "Key derivation error: {}", _0)]
    Crypto(String, Backtrace),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err, Backtrace::new())
    }
}

impl From<ini::ini::Error> for Error {
    fn from(err: ini::ini::Error) -> Error {
        Error::Ini(err, Backtrace::new())
    }
}

impl From<BlockModeError> for Error {
    fn from(err: BlockModeError) -> Error {
        Error::BlockMode(err, Backtrace::new())
    }
}
