use rodio::{OutputStream, OutputStreamHandle, StreamError};

pub(crate) struct AudioDevice {
    _stream: OutputStream,
    pub(crate) handle: OutputStreamHandle,
}

impl AudioDevice {
    pub fn new() -> Result<Self, StreamError> {
        let (_stream, handle) = OutputStream::try_default()?;
        Ok(AudioDevice { _stream, handle })
    }
}
