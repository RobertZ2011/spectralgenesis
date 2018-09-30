extern crate image;

mod audio;
mod fft;
mod process;

#[macro_use]
extern crate clap;
use clap::App;
use std::path::Path;

use fft::Fft;

fn main() {
    let cli_yaml = load_yaml!("cli.yaml");
    let args = App::from_yaml(cli_yaml).get_matches();
    let verbose = args.is_present("verbose");
    let strict = args.is_present("strict");

    let fft = fft::cpu_fft::CpuFft::new(value_t!(args, "width", usize).unwrap());
    if verbose {
        println!("FFT Width: {}", fft.width());
    }

    if let Some(args) = args.subcommand_matches("vis") {
        let audio_in = args.value_of("INPUT").unwrap();
        let image_out = args.value_of("output").unwrap();
        let quanta = if args.is_present("height") {
            value_t!(args, "height", u32).unwrap()
        }
        else {
            value_t!(args, "samples", u32).unwrap() / fft.width()
        };

        let decoder_res = audio::decode_file(Path::new(audio_in), audio::Type::Auto);
        if let Ok(mut decoder) = decoder_res {
            let image_res = process::visualize_decoder(&fft, 0, &mut *decoder, quanta, verbose, strict);

            if let Some(image) = image_res {
                image.save(Path::new(image_out));
            }
            else {
                eprintln!("Falied to create image");
            }
        }
        else {
            eprintln!("Failed to decode audio file");
        }
    }
    else
    if let Some(args) = args.subcommand_matches("embed") {
        let audio_in = args.value_of("INPUT").unwrap();
        let audio_out = args.value_of("output").unwrap();
        let image_in = args.value_of("image").unwrap();

        let decoder_res = audio::decode_file(Path::new(audio_in), audio::Type::Auto);
        let encoder_res = audio::encode_file(Path::new(audio_out), audio::Type::Auto);
        let image_res = image::open(image_in);

        if let (Ok(mut decoder), Ok(mut encoder), Ok(image)) = (decoder_res, encoder_res, image_res) {
            process::process_encoder(&fft, &image, &mut *decoder, &mut *encoder, verbose, strict);
        }
        else {
            eprintln!("Failed to create decoder or encoder");
        }
    }
} 