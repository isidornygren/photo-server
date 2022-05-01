pub mod palette;
use std::{
    error::Error,
    fs::{Metadata, File, self},
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

use ::image::{
    imageops::dither,
    imageops::{crop, resize, FilterType},
    io::Reader,
    ImageOutputFormat,
};
use actix_web::http::{
    header::{ContentDisposition, ContentType, Encoding},
    StatusCode,
};
use serde::Deserialize;

use self::palette::{Palette, EPAPER_PALETTE};

#[derive(Debug, Deserialize)]
pub enum DitherType {
    EPaperSeven,
}

#[derive(Debug, Deserialize)]
pub struct ImageTransformOptions {
    width: Option<u32>,
    height: Option<u32>,
    dither: Option<DitherType>,
}

pub fn load_image<P>(path: P, options: ImageTransformOptions) -> Result<Vec<u8>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    if options.width.is_none() && options.height.is_none() && options.dither.is_none() {
        let mut file = File::open(&path)?;
        let metadata = fs::metadata(&path)?;
        let mut buffer = vec![0; metadata.len() as usize];
        file.read(&mut buffer)?;
        return Ok(buffer);
    }

    let mut img = Reader::open(path)?.decode()?.into_rgb8();

    if options.width.is_some() || options.height.is_some() {
        let width = options
            .width
            .unwrap_or_else(|| img.width() * options.height.unwrap() / img.height());
        let height = options
            .height
            .unwrap_or_else(|| img.height() * width / img.width());

        // Crop the outer edges of the image if the ratio is not correct
        let maybe_ratio = options
            .width
            .and_then(|w| options.height.map(|h| w as f32 / h as f32));
        if let Some(ratio) = maybe_ratio {
            let img_ratio = img.width() as f32 / img.height() as f32;
            // Both width and height was definitely supplied here
            if ratio > img_ratio {
                // wider, take width and then crop
                let resize_height = (1.0 / img_ratio) * options.width.unwrap() as f32;

                img = resize(
                    &img,
                    options.width.unwrap(),
                    resize_height as u32,
                    FilterType::Nearest,
                );

                let y = (resize_height as u32 - options.height.unwrap()) / 2;
                img = crop(
                    &mut img,
                    0,
                    y,
                    options.width.unwrap(),
                    options.height.unwrap(),
                )
                .to_image();
            } else {
                // taller (or same exact ratio), take width and then crop
                let resize_width = img_ratio * options.height.unwrap() as f32;

                img = resize(
                    &img,
                    resize_width as u32,
                    options.height.unwrap(),
                    FilterType::Nearest,
                );
                let x = (resize_width as u32 - options.width.unwrap()) / 2;
                img = crop(
                    &mut img,
                    x,
                    0,
                    options.width.unwrap(),
                    options.height.unwrap(),
                )
                .to_image();
            }
        } else {
            // Only one of either width or height was supplied
            img = resize(&img, width as u32, height as u32, FilterType::Nearest);
        }
    }

    match options.dither {
        Some(DitherType::EPaperSeven) => {
            let palette = Palette::new(EPAPER_PALETTE);
            dither(&mut img, &palette);
        }
        None => {}
    }

    // Write to buffer
    let mut buffer: Vec<u8> = Vec::new();
    let mut writer = Cursor::new(&mut buffer);

    img.write_to(&mut writer, ImageOutputFormat::Png)?;
    return Ok(buffer);
}
