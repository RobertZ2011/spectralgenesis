extern crate image;
extern crate rustfft;

use fft::Fft;
use self::image::{GenericImage, DynamicImage};
use audio::{Decoder, Encoder, Frame};
use self::rustfft::num_complex::Complex32;

pub enum Error {
    OutOfAudio
}

pub fn process_encoder(fft_engine: &Fft, image: &DynamicImage, decoder: &mut Decoder, encoder: &mut Encoder, verbose: bool, strict: bool) {
    let required_samples = (image.height() * fft_engine.width()) as usize;
    let (mut data, rest) = get_samples(required_samples, 0, decoder);
    let acquired_samples = data.len();

    if acquired_samples < required_samples {
        eprintln!("Audio file too small for image");
        return
    }

    let mut freq = vec![Complex32::new(0.0, 0.0); acquired_samples];
    process_slice(fft_engine, image, &mut data.as_mut_slice(), &mut freq.as_mut_slice());

    let samples: Vec<f32> = data.clone().into_iter().map(|c| c.re).collect();
    let frame = Frame {
        data: vec![samples]
    };

    encoder.submit_frame(&frame);
}

//used to hold samples
//(requested samples, remaining samples)
type Samples = (Vec<Complex32>, Option<Vec<Complex32>>);

fn get_samples(required_samples: usize, channel: usize, decoder: &mut Decoder) -> Samples {
    let mut res: Vec<Complex32> = vec![Complex32::new(0.0, 0.0); required_samples];
    let mut samples = 0;
    let mut rest: Option<Vec<Complex32>> = None;

    while samples < required_samples {
        let frame_res = decoder.next_frame();

        if let Ok(frame) = frame_res {
            let len = frame.data[channel].len();
            let data: Vec<Complex32> = frame.data[channel].clone().into_iter().map(|re| Complex32::new(re, 0.0)).collect();

            if samples + len < required_samples {
                //if we can take the whole frame, do it
                
                res.as_mut_slice()[samples .. samples + len].copy_from_slice(data.as_slice());
                samples += len;
            }
            else {
                //otherwise, take what we need
                let to_take = required_samples - samples;
                res.as_mut_slice()[samples .. samples + to_take].copy_from_slice(&data.as_slice()[0 .. to_take]);
                let mut remaining: Vec<Complex32> = vec![Complex32::new(0.0, 0.0); len - to_take];
                remaining.as_mut_slice().copy_from_slice(&data.as_slice()[to_take ..]);
                rest = Some(remaining);
                samples += to_take;
            }
        }
        else {
            break;
        }
    }

    return (res, rest);
}

fn process_slice(fft_engine: &Fft, image: &DynamicImage, audio_time: &mut [Complex32], audio_freq: &mut [Complex32]) {
    let samples = fft_engine.width();
    let quanta = image.height();
    let grayscale = image.grayscale();

    fft_engine.transform(audio_time, audio_freq);

    //embed image into spectrogram
    for i in 0..quanta {
        for j in 0..grayscale.width() {
            let pixel = grayscale.get_pixel(j, i);
            let value = pixel.data[0] as f32 / 255.0;
            audio_freq[(i * samples + j) as usize] += value;
        }
    }

    //mirror and take conjugate of frequecy data
    for j in 0..quanta {
        for i in 0..samples {
            audio_freq[(samples * j + samples - i - 1) as usize] = audio_freq[(samples * j + i) as usize].conj();
        }
    }

    fft_engine.inverse(audio_freq, audio_time);

    //normalize the data
    for sample in audio_time {
        (*sample).re = 2.0 * (*sample).re / (fft_engine.width() as f32);
        (*sample).im = 0.0;
    }
}

pub fn visualize_decoder(fft_engine: &Fft, channel: usize, decoder: &mut Decoder, quanta: u32, verbose: bool, strict: bool) -> Option<DynamicImage> {
    let required_samples = (fft_engine.width() * quanta) as usize;
    let (mut audio_time, _) = get_samples(required_samples, channel, decoder);
    let acquired_samples = audio_time.len();

    //strict mode requires that we have enough data
    if strict {
        if acquired_samples != required_samples {
            if verbose {
                eprintln!("Strict failure: not enough samples, required: {}, acquired: {}", required_samples, acquired_samples);
            }
            return None
        }
    }

    let mut audio_freq = vec![Complex32::new(0.0, 0.0); acquired_samples];
    visualize_slice(fft_engine, &mut audio_time, &mut audio_freq, quanta)
}

fn visualize_slice(fft_engine: &Fft, audio_time: &mut [Complex32], audio_freq: &mut [Complex32], quanta: u32) -> Option<DynamicImage> {
    let samples = fft_engine.width();
    //we're dealing with real signals
    //the other half of the image is just mirrored
    let img_width = samples / 2;

    if let DynamicImage::ImageLuma8(mut image) = DynamicImage::new_luma8(img_width, quanta) {
        fft_engine.transform(audio_time, audio_freq);

        //get amplitude of data
        let audio_amp: Vec<f32> = audio_freq.to_vec().into_iter().map(|c| c.norm()).collect();

        //get max audio value
        let mut max_audio: f32 = 0.0;
        for sample in &audio_amp {
            if *sample > max_audio {
                max_audio = *sample;
            }
        }

        for i in 0..quanta {
            for j in 0..img_width {
                let index = (i * samples + j) as usize;
                let value = audio_amp[index];
                let pixel: image::Luma<u8> = image::Luma {
                    data: [(20.0 * (value / max_audio).log10()) as u8; 1]
                };
                image.put_pixel(j, i, pixel);
            }
        }
        
        Some(DynamicImage::ImageLuma8(image))
    }
    else {
        None
    }
}