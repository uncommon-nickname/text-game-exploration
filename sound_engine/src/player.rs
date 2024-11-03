use super::sound::Sound;

use pyo3::exceptions::PyEnvironmentError;
use pyo3::prelude::{pyclass, pymethods, PyResult};
use rodio::{OutputStream, OutputStreamHandle};

#[pyclass(unsendable)]
pub(crate) struct AudioPlayer {
    _stream: OutputStream,
    handle: OutputStreamHandle,
}

#[pymethods]
impl AudioPlayer {
    #[new]
    fn new() -> PyResult<Self> {
        let (_stream, handle) = OutputStream::try_default()
            .map_err(|_| PyEnvironmentError::new_err("Cannot initialize output stream."))?;

        Ok(AudioPlayer { _stream, handle })
    }

    fn new_sound(&self) -> PyResult<Sound> {
        let sound = Sound::new(&self.handle)
            .map_err(|_| PyEnvironmentError::new_err("Cannot initialize new sink."))?;

        Ok(sound)
    }
}
