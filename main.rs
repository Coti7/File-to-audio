use std::fmt::{format, write};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use hound;
use std::f32::consts::PI;

fn main() -> std::io::Result<()> {
    let file = "img.png";
    
    let path = Path::new(file);
    let filename = path.file_stem().unwrap().to_str().unwrap();
    let ext = path.extension().unwrap().to_str().unwrap();
    
    let ext_byte: Vec<u8> = ext.as_bytes().to_vec();
    let mut frequencies: Vec<Vec<f32>> = Vec::new();

    let mut f = File::open(file).unwrap();
    let mut byte = Vec::new();
    f.read_to_end(&mut byte).unwrap();

    println!("Generating sound...");

    for &b in &byte {
        frequencies.push(generate_frequency((b as f32 * 10.0) + 100.0, 0.1, 7500, 0.5));
    }

    for &b in &ext_byte {
        frequencies.push(generate_frequency((b as f32 * 10.0) + 100.0, 0.1, 7500, 0.5));
    }

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 7500,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    let mut writer = hound::WavWriter::create(format!("{}.wav", filename), spec).unwrap();
    
    println!("Writing file...");

    for frequency in frequencies {
        let wave_data: Vec<i16> = frequency.iter().map(|&x| (x * 32767.0) as i16).collect();
        
        for sample in wave_data {
            writer.write_sample(sample).unwrap();
        }
    }
    writer.finalize().unwrap();

    Ok(())
}


fn generate_frequency(frequency: f32, duration: f32, sample_rate: u32, amplitude: f32) -> Vec<f32> {
    let t: Vec<f32> = (0..(sample_rate as f32 * duration) as usize)
        .map(|x| x as f32 / sample_rate as f32)
        .collect();

    t.iter()
        .map(|&x| amplitude * (2.0 * PI * frequency * x).sin())
        .collect()
}