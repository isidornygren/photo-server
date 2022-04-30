pub mod converter;
use std::{io::{Cursor}, error::Error, path::Path};

use ::image::{
    imageops::dither,
    imageops::{crop, resize, FilterType},
    io::Reader,
    ImageOutputFormat,
};
use serde::Deserialize;

use self::converter::{EPAPER_PALETTE, Palette};

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

pub fn load_image<P>(path: P, image_transform_options: ImageTransformOptions) -> Result<Vec<u8>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut img = Reader::open(path)?.decode()?.into_rgb8();

    if image_transform_options.width.is_some() || image_transform_options.height.is_some() {
        let width = image_transform_options
            .width
            .unwrap_or_else(|| img.width() * image_transform_options.height.unwrap() / img.height());
        let height = image_transform_options
            .height
            .unwrap_or_else(|| img.height() * width / img.width());

        // Crop the outer edges of the image if the ratio is not correct
        let maybe_ratio = image_transform_options
            .width
            .and_then(|w| image_transform_options.height.map(|h| w as f32 / h as f32));
        if let Some(ratio) = maybe_ratio {
            let img_ratio = img.width() as f32 / img.height() as f32;
            // Both width and height was definitely supplied here
            if ratio > img_ratio {
                // wider, take width and then crop
                let resize_height = (1.0 / img_ratio) * image_transform_options.width.unwrap() as f32;

                img = resize(
                    &img,
                    image_transform_options.width.unwrap(),
                    resize_height as u32,
                    FilterType::Nearest,
                );

                let y = (resize_height as u32 - image_transform_options.height.unwrap()) / 2;
                img = crop(&mut img, 0, y, image_transform_options.width.unwrap(), image_transform_options.height.unwrap()).to_image();
            } else {
                // taller (or same exact ratio), take width and then crop
                let resize_width = img_ratio * image_transform_options.height.unwrap() as f32;

                img = resize(
                    &img,
                    resize_width as u32,
                    image_transform_options.height.unwrap(),
                    FilterType::Nearest,
                );
                let x = (resize_width as u32 - image_transform_options.width.unwrap()) / 2;
                img = crop(&mut img, x, 0, image_transform_options.width.unwrap(), image_transform_options.height.unwrap()).to_image();
            }
        } else {
            // Only one of either width or height was supplied
            img = resize(&img, width as u32, height as u32, FilterType::Nearest);
        }
    }

    match image_transform_options.dither {
        Some(DitherType::EPaperSeven) => {
            let palette = Palette::new(EPAPER_PALETTE);
            dither(&mut img, &palette);
        }
        _ => {}
    }

    // Write to buffer
    let mut buffer: Vec<u8> = Vec::new();
    let mut writer = Cursor::new(&mut buffer);

    img.write_to(&mut writer, ImageOutputFormat::Png)?;
    return Ok(buffer);
}
