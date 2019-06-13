use std::env;
use std::io;

fn main() -> Result<(), chttp::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let mut response = chttp::post("https://httpbin.org/post", ())?;

    let mut sink = io::sink();
    io::copy(response.body_mut(), &mut sink)?;

    Ok(())
}
