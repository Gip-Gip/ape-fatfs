use std::env;
use std::fs;
use std::io;

use ape_fatfs::{
    fs::{format_volume, FormatVolumeOptions},
    io::StdIoWrapper,
};
use fscommon::BufStream;

fn main() -> io::Result<()> {
    let filename = env::args().nth(1).expect("image path expected");
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&filename)?;
    let buf_file = BufStream::new(file);
    format_volume(
        &mut StdIoWrapper::from(buf_file),
        FormatVolumeOptions::new(),
    )?;
    Ok(())
}
