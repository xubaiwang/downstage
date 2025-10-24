use std::io;

use webdriverbidi::error::{CommandError, SessionError};

// TODO: should group by functionlity, not source
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("command")]
    Command(#[from] CommandError),
    #[error("session")]
    Session(#[from] SessionError),
    #[error("io")]
    Io(#[from] io::Error),
    #[error("fail to spawn")]
    Spawn,
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
