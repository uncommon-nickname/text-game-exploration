use super::device::AudioDevice;

use pyo3::exceptions::{PyEnvironmentError, PyFileNotFoundError};
use pyo3::prelude::{pyclass, pymethods, PyResult};
use rodio::{Decoder, PlayError, Sink, Source};

use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;

#[pyclass(unsendable)]
pub(crate) struct AudioPlayer {
    // This ensures that device reference will always live as long as
    // any audio player reference is alive.
    _device: Rc<AudioDevice>,
    sink: Sink,
}

impl AudioPlayer {
    pub(crate) fn new(device: Rc<AudioDevice>) -> Result<Self, PlayError> {
        let sink = Sink::try_new(&device.handle)?;

        sink.pause();

        Ok(AudioPlayer {
            _device: device,
            sink,
        })
    }
}

#[pymethods]
impl AudioPlayer {
    #[pyo3(signature = (path, repeat = false))]
    fn queue_sound(&self, path: String, repeat: bool) -> PyResult<()> {
        let file = File::open(path).map_err(|_| PyFileNotFoundError::new_err("File not found."))?;
        let buff = BufReader::new(file);

        let source = Decoder::new(buff)
            .map_err(|_| PyEnvironmentError::new_err("Cannot initialize decoder."))?;

        if repeat {
            self.sink.append(source.repeat_infinite());
        } else {
            self.sink.append(source);
        }

        Ok(())
    }

    #[pyo3(signature = (volume = 1.0))]
    fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    fn play(&self) {
        self.sink.play();
    }

    fn pause(&self) {
        self.sink.pause();
    }

    fn stop(&self) {
        self.sink.stop();
    }
}
