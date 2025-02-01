use std::fs::File;
use std::io::{self, BufReader, Read};

struct StdfReader {
    reader: BufReader<File>,
    buffer_size: usize,
}

impl StdfReader {
    fn new(file_path: &str, buffer_size: usize) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        Ok(StdfReader { reader, buffer_size })
    }

    fn read_record(&mut self) -> io::Result<Vec<u8>> {
        let mut buffer = vec![0; self.buffer_size];
        let bytes_read = self.reader.read(&mut buffer)?;
        buffer.truncate(bytes_read);
        Ok(buffer)
    }
}

fn main() -> io::Result<()> {
    let file_path = "path/to/your/file.txt";
    let buffer_size = 1024; // Read in chunks of 1024 bytes

    let mut chunk_reader = StdfReader::new(file_path, buffer_size)?;

    loop {
        let chunk = chunk_reader.read_record()?;
        if chunk.is_empty() {
            break;
        }
        println!("Read chunk: {:?}", chunk);
    }

    Ok(())
}