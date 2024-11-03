use pyo3::exceptions::{PyEnvironmentError, PyFileNotFoundError};
use pyo3::prelude::{pyclass, pymethods, PyResult};
use rodio::{Decoder, OutputStreamHandle, PlayError, Sink, Source};

use std::fs::File;
use std::io::BufReader;

#[pyclass]
pub(crate) struct Sound {
    sink: Sink,
}

impl Sound {
    pub(crate) fn new(handle: &OutputStreamHandle) -> Result<Self, PlayError> {
        let sink = Sink::try_new(handle)?;
        sink.pause();

        Ok(Sound { sink })
    }
}

#[pymethods]
impl Sound {
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

    #[pyo3(signature = (volume=1.0))]
    fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    fn play(&self) {
        self.sink.play();
    }

    fn pause(&self) {
        self.sink.pause();
    }
}
