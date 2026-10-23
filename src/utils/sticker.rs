use image::{
    DynamicImage, ImageEncoder, RgbaImage,
    codecs::png::{CompressionType, FilterType as PngFilter, PngEncoder},
    imageops::{self, FilterType},
};

pub const STICKER_SIZE: u32 = 320;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CropMode {
    #[default]
    Fill,
    Fit,
    Dented,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StickerOptions {
    pub crop: CropMode,
    pub center: bool,
}

impl Default for StickerOptions {
    fn default() -> Self {
        Self {
            crop: CropMode::default(),
            center: true,
        }
    }
}

pub fn to_sticker(
    bytes: &[u8],
    options: StickerOptions,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let source = image::load_from_memory(bytes)?;

    let canvas = match options.crop {
        CropMode::Fill => fill(&source, options.center),
        CropMode::Fit => fit(&source, options.center),
        CropMode::Dented => source
            .resize_exact(STICKER_SIZE, STICKER_SIZE, FilterType::Lanczos3)
            .to_rgba8(),
    };

    let mut encoded = Vec::new();

    PngEncoder::new_with_quality(&mut encoded, CompressionType::Best, PngFilter::Adaptive)
        .write_image(
            canvas.as_raw(),
            STICKER_SIZE,
            STICKER_SIZE,
            image::ExtendedColorType::Rgba8,
        )?;

    Ok(encoded)
}

fn fill(source: &DynamicImage, center: bool) -> RgbaImage {
    let scale = f64::max(
        f64::from(STICKER_SIZE) / f64::from(source.width()),
        f64::from(STICKER_SIZE) / f64::from(source.height()),
    );
    let width = ((f64::from(source.width()) * scale).round() as u32).max(STICKER_SIZE);
    let height = ((f64::from(source.height()) * scale).round() as u32).max(STICKER_SIZE);

    let scaled = source
        .resize_exact(width, height, FilterType::Lanczos3)
        .to_rgba8();

    let (x, y) = offset(center, width - STICKER_SIZE, height - STICKER_SIZE);

    imageops::crop_imm(&scaled, x, y, STICKER_SIZE, STICKER_SIZE).to_image()
}

fn fit(source: &DynamicImage, center: bool) -> RgbaImage {
    let scaled = source
        .resize(STICKER_SIZE, STICKER_SIZE, FilterType::Lanczos3)
        .to_rgba8();

    let mut canvas = RgbaImage::new(STICKER_SIZE, STICKER_SIZE);
    let (x, y) = offset(
        center,
        STICKER_SIZE - scaled.width(),
        STICKER_SIZE - scaled.height(),
    );
    imageops::overlay(&mut canvas, &scaled, i64::from(x), i64::from(y));

    canvas
}

fn offset(center: bool, horizontal_space: u32, vertical_space: u32) -> (u32, u32) {
    if center {
        (horizontal_space / 2, vertical_space / 2)
    } else {
        (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use image::{ImageFormat, Rgba};

    use super::*;

    fn half_red_half_blue(width: u32, height: u32) -> Vec<u8> {
        let mut image = RgbaImage::new(width, height);

        for (x, _, pixel) in image.enumerate_pixels_mut() {
            *pixel = if x < width / 2 {
                Rgba([255, 0, 0, 255])
            } else {
                Rgba([0, 0, 255, 255])
            };
        }

        let mut bytes = std::io::Cursor::new(Vec::new());
        image.write_to(&mut bytes, ImageFormat::Png).unwrap();
        bytes.into_inner()
    }

    fn sticker(options: StickerOptions) -> RgbaImage {
        let png = to_sticker(&half_red_half_blue(400, 200), options).unwrap();

        image::load_from_memory(&png).unwrap().to_rgba8()
    }

    fn is_red(pixel: &Rgba<u8>) -> bool {
        pixel[0] > 128 && pixel[2] < 128
    }

    fn is_blue(pixel: &Rgba<u8>) -> bool {
        pixel[2] > 128 && pixel[0] < 128
    }

    fn options(crop: CropMode, center: bool) -> StickerOptions {
        StickerOptions { crop, center }
    }

    #[test]
    fn defaults_to_a_centered_fill() {
        assert_eq!(StickerOptions::default(), options(CropMode::Fill, true));
    }

    #[test]
    fn fill_centered_keeps_both_halves() {
        let result = sticker(options(CropMode::Fill, true));

        assert_eq!(result.dimensions(), (320, 320));
        assert!(is_red(result.get_pixel(10, 160)));
        assert!(is_blue(result.get_pixel(310, 160)));
    }

    #[test]
    fn fill_anchored_keeps_only_the_left_side() {
        let result = sticker(options(CropMode::Fill, false));

        assert!(is_red(result.get_pixel(10, 160)));
        assert!(is_red(result.get_pixel(310, 160)));
    }

    #[test]
    fn fit_adds_transparent_bars_around_the_image() {
        let centered = sticker(options(CropMode::Fit, true));
        let anchored = sticker(options(CropMode::Fit, false));

        assert_eq!(centered.get_pixel(160, 5)[3], 0);
        assert!(centered.get_pixel(160, 160)[3] > 0);
        assert!(anchored.get_pixel(160, 5)[3] > 0);
        assert_eq!(anchored.get_pixel(160, 310)[3], 0);
    }

    #[test]
    fn dented_stretches_the_whole_image() {
        let result = sticker(options(CropMode::Dented, true));

        assert!(is_red(result.get_pixel(10, 5)));
        assert!(is_blue(result.get_pixel(310, 316)));
    }

    #[test]
    fn rejects_data_that_is_not_an_image() {
        assert!(to_sticker(b"not an image", StickerOptions::default()).is_err());
    }
}
