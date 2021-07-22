mod requestor;
mod vibaimage;
#[tokio::main]
async fn main() {

    let x = std::fs::File::open("/tmp/x.jpeg").unwrap();

    let i = vibaimage::Image::from_read(x).unwrap();

    i.write(std::fs::File::create("/tmp/y.jpeg").unwrap()).unwrap();
}
