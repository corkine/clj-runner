use std::io;
#[cfg(windows)] use winres::WindowsResource;
//https://stackoverflow.com/questions/30291757/attaching-an-icon-resource-to-a-rust-application
fn main() -> io::Result<()> {
    #[cfg(windows)] {
        WindowsResource::new()
            .set_icon("../favicon.ico")
            .compile()?;
    }
    Ok(())
}