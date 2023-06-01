use clap::Parser;
use image::{save_buffer, DynamicImage, GenericImageView, Rgb, RgbImage, Rgba};
use lerp::Lerp;
use rand::Rng;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// input filename. (string)
    #[arg(short = 'f', long,  value_name = "FILENAME", value_hint = clap::ValueHint::FilePath)]
    filename: String,

    /// kernel size for sampling. (float)
    #[arg(short = 'i', long, value_name = "INTENSITY")]
    intensity: f32,

    /// How much it effects the luminosity. (float)
    #[arg(short = 'l', long, value_name = "lumen")]
    lumen: f32,
}

// generates noise.
fn generate_noise(width: u32, height: u32, noise: &mut Vec<Vec<f32>>) {
    let mut rng = rand::thread_rng();
    for _ in 0..height {
        let mut local_noise: Vec<f32> = Vec::new();
        for _ in 0..width {
            let r: f32 = rng.gen_range(0.0..1.0);
            local_noise.push(r);
        }
        noise.push(local_noise);
    }
}

// calculates lumen from rgb
fn calculate_lumen(color: (f32, f32, f32)) -> f32 {
    0.2126 * color.0 + 0.7152 * color.1 + 0.0722 * color.2
}

//normalizes values between 0 and 1.
fn normalize(value: u8) -> f32 {
    (value as f32 - 0.0) / (255.0 - 0.0)
}

// calculate final value.
fn calculate_pixel(pixel: Rgba<u8>, weight: f32, intensity: f32, noise_value: f32) -> Rgb<u8> {
    let r = (pixel.0[0] as f32 * 2.0) * weight * intensity * noise_value;
    let g = (pixel.0[1] as f32 * 2.0) * weight * intensity * noise_value;
    let b = (pixel.0[2] as f32 * 2.0) * weight * intensity * noise_value;
    Rgb([r as u8, g as u8, b as u8])
}

fn main() {
    let args = Args::parse();

    // opening image
    println!("Reading Image : {}", args.filename);
    let original_image: DynamicImage = match image::open(Args::parse().filename) {
        Ok(result) => result,
        Err(err) => {
            println!("[ERROR] {}", err);
            exit(1);
        }
    };

    // generating noise.
    println!(
        "Generating noise for dimensions : {}x{}",
        original_image.width(),
        original_image.height()
    );
    let mut noise: Vec<Vec<f32>> = Vec::new();
    generate_noise(original_image.width(), original_image.height(), &mut noise);

    // result image buffer.
    let mut new_image = RgbImage::new(original_image.width(), original_image.height());

    // looping through pixels.
    for y in 0..original_image.height() {
        for x in 0..original_image.width() {
            let pixel = original_image.get_pixel(x, y);
            let pixel_lumen = calculate_lumen((
                normalize(pixel.0[0]),
                normalize(pixel.0[1]),
                normalize(pixel.0[2]),
            ));
            let mut weight = 1.0 - f32::sqrt(pixel_lumen);
            weight = 1.0_f32.lerp(weight, args.lumen);

            let new_pixel =
                calculate_pixel(pixel, weight, args.intensity, noise[y as usize][x as usize]);

            new_image.put_pixel(x, y, new_pixel);

            // println!("{:?} -> {:?}", pixel, new_pixel);
        }
    }

    // saving buffer to disk.
    match save_buffer(
        "result.jpg",
        &new_image,
        original_image.width(),
        original_image.height(),
        image::ColorType::Rgb8,
    ) {
        Err(err) => println!("[ERROR] - while saving - {}", err),
        Ok(result) => result,
    };
    println!("Done writing.");
}
