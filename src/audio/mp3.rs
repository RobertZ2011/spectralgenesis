extern crate minimp3;

use std::path::Path;
use std::fs::File;

use audio;
use audio::DecodeError;

pub struct Decoder {
    decoder: minimp3::Decoder<File>,
}

impl Decoder {
    pub fn open(file: &Path) -> Result<Box<audio::Decoder>, DecodeError> {
        let result = File::open(file);
        if let Err(_) = result {
            //TODO: actual error handling
            return Err(DecodeError::Eof);
        }

        let reader = result.unwrap();
        Ok(Box::new(Decoder{
            decoder: minimp3::Decoder::new(reader)
        }))
    }
}

impl audio::Decoder for Decoder {
    fn next_frame(&mut self) -> Result<audio::Frame, DecodeError> {
        let frame_result = self.decoder.next_frame();

        if let Err(err) = frame_result {
            return match err {
                minimp3::Error::Eof => Err(DecodeError::Eof),
                minimp3::Error::SkippedData => self.next_frame(), //try and get the next frame
                minimp3::Error::InsufficientData => Err(DecodeError::IncompleteData),
                minimp3::Error::Io(io_err) => Err(DecodeError::Io(io_err))
            }
        }

        let frame = frame_result.unwrap();
        if frame.channels > 255 {
            return Err(DecodeError::TooManyChannels);
        }
        
        let mut channels: Vec<Vec<f32>> = Vec::with_capacity(frame.channels);
        let samples = frame.data.len() / frame.channels;

        for _ in 0..frame.channels {
            channels.push(Vec::with_capacity(samples));
        }

        for i in 0..samples {
            for j in 0..frame.channels {
                let float: f32 = frame.data[i * frame.channels + j] as f32 / 32_767.0;
                channels[j].push(float);
            }
        }

        Ok(audio::Frame{
            data: channels
        })
    }
}