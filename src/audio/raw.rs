use audio::{Frame, DecodeError};
use audio;

use std::path::Path;
use std::fs::File;

use std::io::{BufReader, BufWriter, Write};

struct Decoder {
    reader: BufReader<File>
}

pub struct Encoder {
    writer: BufWriter<File>
}

impl Decoder {
    fn create(path: &Path) -> Result<Box<audio::Decoder>, DecodeError> {
        Err(DecodeError::UnknownFormat)
    }
}

impl audio::Decoder for Decoder {
    fn next_frame(&mut self) -> Result<Frame, DecodeError> {
        /*//read up to 1000000 samples
        let mut samples: Vec<f32> = Vec::with_capacity(1_000_000);
        let mut line = String::new();

        for i in 0..1_000_000 {
            let res = self.reader.read_line(&mut line);

            if let Ok(line) = res {
                let sample_res = line.parse()
                if let Ok(sample: f32) = sample_res {
                    samples.push(sample);
                }
                else {
                    let err = sample_res.unwrap_err();
                }
            }
            else {

            }
        }*/
        Err(DecodeError::Eof)
    }
}

impl Encoder {
    pub fn create(path: &Path) -> Result<Box<audio::Encoder>, DecodeError> {
        //TODO: actual error handling
        let file = File::create(path).unwrap();
        Ok(Box::new(Encoder {
            writer: BufWriter::new(file)
        }))
    }
}

impl audio::Encoder for Encoder {
    fn submit_frame(&mut self, frame: &Frame) {
        for sample in &frame.data[0] {
            write!(&mut self.writer, "{}\n", *sample);
        }
    }
}