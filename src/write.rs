
use std::io::{self, BufWriter, ErrorKind, Result, Write};
use std::fs::File;

pub fn write(outfile: &str, buffer: &[u8]) -> Result<bool> {
    let mut writer : Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new( File::create(outfile)?))
    }else {
        Box::new(io::stdout())
    };

    if let Err(e) = writer.write_all(&buffer){
        if e.kind() == ErrorKind::BrokenPipe {
            return Ok(false)
        }
        eprintln!("the pipe was broken");
        std::process::exit(1);
    }
    Ok(true)
}