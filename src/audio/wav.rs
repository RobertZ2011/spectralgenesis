extern crate hound;

use std::path::Path;
use std::fs::File;

use audio;
use audio::DecodeError;

pub struct Encoder {
    encoder: hound::WavWriter<File>
}

pub struct Decoder {
    decoder: hound::WavReader<File>
}

impl Encoder {
    pub fn open(path: &Path, channels: u8, sample_rate: u32) -> Result<Box<audio::Encoder>, DecodeError> {
        let spec = hound::WavSpec {
            channels: channels as u16,
            sample_rate: sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int
        };

        let result = File::create(path);
        if let Err(e) = result {
            println!("{}", e);
            //TODO: actual error handiing
            return Err(DecodeError::Eof);
        }

        let writer = result.unwrap();
        let enc_res = hound::WavWriter::new(writer, spec);

        if let Err(_) = enc_res {
            //TODO: actual error handling
            return Err(DecodeError::Eof);
        }

        Ok(Box::new(Encoder{
            encoder: enc_res.unwrap()
        }))

    }
}

impl audio::Encoder for Encoder {
    fn submit_frame(&mut self, frame: &audio::Frame) {
        for i in 0..frame.data[0].len() {
            for j in 0..frame.data.len() {
                self.encoder.write_sample((frame.data[j][i] * 32_767.0) as i16);
            } 
        }
    }
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
            decoder: hound::WavReader::new(reader).unwrap()
        }))
    }
}

impl audio::Decoder for Decoder {
    fn next_frame(&mut self) -> Result<audio::Frame, DecodeError> {
        let spec = self.decoder.spec();
        let mut reader = self.decoder.samples();
        let mut data: Vec<i16> = Vec::with_capacity(reader.len());

        while let Some(v) = reader.next() {
            if v.is_err() {
                return Err(DecodeError::Eof);
            }
            else {
                data.push(v.unwrap());
            }
        }
        
        let mut channels: Vec<Vec<f32>> = Vec::with_capacity(spec.channels as usize);
        let samples = data.len() / spec.channels as usize;

        if samples == 0 {
            return Err(DecodeError::Eof);
        }

        for _ in 0..spec.channels {
            channels.push(Vec::with_capacity(samples));
        }

        for i in 0..samples {
            for j in 0..spec.channels {
                let index = i * spec.channels as usize + j as usize;
                let float: f32 = data[index] as f32 / 32_767.0;
                channels[j as usize].push(float);
            }
        }

        Ok(audio::Frame{
            data: channels
        })
    }
}