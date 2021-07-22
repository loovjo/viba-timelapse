#[derive(Debug)]
pub enum VibaError {
    ImageError(image::ImageError),
    HyperError(hyper::Error),
    IOError(std::io::Error),
}

impl From<image::ImageError> for VibaError {
    fn from(e: image::ImageError) -> VibaError {
        VibaError::ImageError(e)
    }
}

impl From<hyper::Error> for VibaError {
    fn from(e: hyper::Error) -> VibaError {
        VibaError::HyperError(e)
    }
}

impl From<std::io::Error> for VibaError {
    fn from(e: std::io::Error) -> VibaError {
        VibaError::IOError(e)
    }
}
