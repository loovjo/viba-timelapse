use image::ImageDecoder;

pub struct Image {
    pub width: usize,
    pub data: Vec<(f64, f64, f64)>,
}

impl Image {
    pub fn from_read<R: std::io::Read>(r: R) -> image::ImageResult<Image> {
        let dec = image::codecs::jpeg::JpegDecoder::new(r)?;
        let (width, _height) = dec.dimensions();

        if dec.color_type() != image::ColorType::Rgb8 {
            let hint =
                image::error::ImageFormatHint::Name(format!("JPEG with {:?}", dec.color_type()));

            return Err(image::ImageError::Unsupported(
                image::error::UnsupportedError::from_format_and_kind(
                    hint.clone(),
                    image::error::UnsupportedErrorKind::Format(hint),
                ),
            ));
        }

        let mut raw_data = vec![0; dec.total_bytes() as usize];

        dec.read_image(&mut raw_data)?;

        let mut data = Vec::with_capacity(raw_data.len() / 3);
        for i in 0..raw_data.len() / 3 {
            let (r, g, b) = (raw_data[3 * i], raw_data[3 * i + 1], raw_data[3 * i + 2]);
            data.push((r as f64, g as f64, b as f64));
        }

        Ok(Image {
            width: width as usize,
            data,
        })
    }

    pub fn write<W: std::io::Write>(self, mut w: W) -> image::ImageResult<()> {
        let mut enc = image::codecs::jpeg::JpegEncoder::new(&mut w);

        let mut data = Vec::with_capacity(self.data.len() * 3);
        let height = self.data.len() / self.width;
        for (r, g, b) in self.data {
            data.push(r as u8);
            data.push(g as u8);
            data.push(b as u8);
        }
        enc.encode(
            &data,
            self.width as u32,
            height as u32,
            image::ColorType::Rgb8,
        )
    }
}
