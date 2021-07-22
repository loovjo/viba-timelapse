use crate::vibaimage::{Image, avg_images};
use crate::error::VibaError;
use hyper::body::HttpBody;
use futures::prelude::*;

pub struct Requestor<C: hyper::client::connect::Connect> {
    pub img_url: hyper::Uri,

    pub client: hyper::client::Client<C>,
    pub emit_path: String,

    pub time_between_images: tokio::time::Duration,
    pub min_time_between_reqs: tokio::time::Duration,
}

impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static> Requestor<C> {
    pub async fn load_one_image(&self) -> Result<Image, VibaError> {
        let resp = self.client.get(self.img_url.clone()).await?;

        assert_eq!(resp.status(), hyper::StatusCode::from_u16(200).unwrap());

        let mut body: hyper::body::Body = resp.into_body();

        let mut data: Vec<u8> = Vec::new();
        while let Some(chunk) = body.data().await {
            let chunk: hyper::body::Bytes = chunk?;
            data.extend(chunk);
        }

        return Ok(Image::from_read(std::io::Cursor::new(data))?);
    }

    // Will take the self.time_between_images time
    // (only if the first request doesn't take more time than that)
    //
    // returns vector of images and a number represeting number of requests which wre not delayed
    pub async fn get_n_images(&mut self) -> Result<(Vec<Image>, usize), VibaError> {
        let start = tokio::time::Instant::now();
        let deadline = start + self.time_between_images;

        let mut images: Vec<Image> = Vec::new();

        images.push(self.load_one_image().await?);

        let mut n_nonsleeps = 0;

        loop {
            if tokio::time::Instant::now() > deadline {
                break;
            }

            let max_time_left = deadline - tokio::time::Instant::now();
            let start = tokio::time::Instant::now();

            tokio::select!(
                _ = tokio::time::sleep(max_time_left) => {
                    break;
                }
                i = self.load_one_image() => {
                    images.push(i?);
                    let next_req_time = start + self.min_time_between_reqs;
                    if tokio::time::Instant::now() > next_req_time {
                        self.min_time_between_reqs *= 2;
                        n_nonsleeps += 1;
                    } else {
                        self.min_time_between_reqs -= tokio::time::Duration::from_millis(50);
                        tokio::time::sleep_until(next_req_time).await;
                    }
                }
            );
        }
        Ok((images, n_nonsleeps))
    }

    pub async fn run(&mut self) -> Result<(), VibaError> {
        let mut last_images: Option<Vec<Image>> = None;

        loop {
            let path = self.emit_path.clone();

            let res = tokio::try_join!(
                self.get_n_images(),
                tokio::task::spawn_blocking({
                    let images = last_images.take();
                    move || -> Result<(), VibaError> {
                        if let Some(last_images) = images {
                            let img = avg_images(last_images);

                            let timestamp = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).map_err(|_| VibaError::TimeStampError)?;

                            let f = std::fs::File::create(format!("{}/{}.jpeg", path, timestamp.as_millis()))?;

                            img.write(f)?;
                        }
                        Ok(())
                    }
                }).map(|r| r.unwrap_or(Err(VibaError::JoinError))),
            );
            match res {
                Ok(((imgs, n_nonsleeps), ())) => {
                    println!("Collected {} images, nonsleeps = {}, delay = {:?}", imgs.len(), n_nonsleeps, self.min_time_between_reqs);

                    last_images = Some(imgs);
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                }
            }
        }
    }
}
