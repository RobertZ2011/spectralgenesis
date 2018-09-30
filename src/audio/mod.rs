mod mp3;
mod wav;
mod raw;

use std;
use std::ffi::OsStr;

#[derive(Debug)]
pub enum DecodeError {
    UnknownFormat,
    Eof,
    TooManyChannels,
    IncompleteData,
    Io(std::io::Error)
}

pub enum Type {
    Auto,
    Mp3,
    Wav,
    Txt,
}

pub struct Frame {
    pub data: Vec<Vec<f32>>,
}

pub trait Encoder {
    fn submit_frame(&mut self, frame: &Frame);
}

pub trait Decoder {
    fn next_frame(&mut self) -> Result<Frame, DecodeError>;
}

pub fn decode_file(path: &std::path::Path, audio_type: Type) -> Result<Box<Decoder>, DecodeError> {
    match audio_type {
        Type::Mp3  => mp3::Decoder::open(path),
        Type::Wav  => wav::Decoder::open(path),
        Type::Txt  => Err(DecodeError::UnknownFormat),
        Type::Auto => {
            let ext = path.extension().unwrap_or(OsStr::new("")).to_str().unwrap_or("");
            match ext {
                "mp3" => decode_file(path, Type::Mp3),
                "wav" => decode_file(path, Type::Wav),
                _           => Err(DecodeError::UnknownFormat)
            }
        }
    }
}

pub fn encode_file(path: &std::path::Path, audio_type: Type) -> Result<Box<Encoder>, DecodeError> {
    match audio_type {
        Type::Txt => raw::Encoder::create(path),
        Type::Auto => {
            let ext = path.extension().unwrap_or(OsStr::new("")).to_str().unwrap_or("");
            match ext {
                "txt" => encode_file(path, Type::Txt),
                _           => Err(DecodeError::UnknownFormat)
            }
        },
        _ => Err(DecodeError::UnknownFormat)
    }
}