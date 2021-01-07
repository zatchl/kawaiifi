use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("Interface is already scanning")]
    AlreadyScanning,

    #[error("Permission denied")]
    PermissionDenied,

    #[cfg(target_os = "linux")]
    #[error(transparent)]
    NetlinkError(#[from] neli::err::NlError),

    #[cfg(target_os = "linux")]
    #[error(transparent)]
    NetlinkSerializationError(#[from] neli::err::SerError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
