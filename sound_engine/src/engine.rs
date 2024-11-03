use super::device::AudioDevice;
use super::player::AudioPlayer;

use pyo3::exceptions::PyEnvironmentError;
use pyo3::prelude::{pyclass, pymethods, PyResult};

use std::rc::Rc;

#[pyclass(unsendable)]
pub(crate) struct AudioEngine {
    device: Rc<AudioDevice>,
}

#[pymethods]
impl AudioEngine {
    #[new]
    fn new() -> PyResult<Self> {
        let device = AudioDevice::new()
            .map_err(|_| PyEnvironmentError::new_err("Cannot initialize output stream."))?;

        Ok(AudioEngine {
            device: Rc::new(device),
        })
    }

    fn new_concurrent_player(&self) -> PyResult<AudioPlayer> {
        let sound = AudioPlayer::new(self.device.clone())
            .map_err(|_| PyEnvironmentError::new_err("Cannot initialize new sink."))?;

        Ok(sound)
    }
}
