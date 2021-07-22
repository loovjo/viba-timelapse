use crate::vibaimage::Image;

pub struct Requestor {
    img_url: String,
    emit_path: String,
}

impl Requestor {
    async fn load_one_image(&self) -> image::ImageResult<Image> {
        unimplemented!()
    }
}
