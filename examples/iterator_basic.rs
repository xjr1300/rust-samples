use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Result as IoResult};
use std::path::Path;

struct FileLines {
    reader: BufReader<File>,
    buffer: String,
}

impl FileLines {
    fn new<P>(path: P) -> IoResult<Self>
    where
        P: AsRef<Path>,
    {
        let file = OpenOptions::new().read(true).open(path)?;

        Ok(Self {
            reader: BufReader::new(file),
            buffer: String::new(),
        })
    }
}

impl Iterator for FileLines {
    type Item = IoResult<String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        match self.reader.read_line(&mut self.buffer) {
            Ok(0) => None,
            Ok(_) => Some(Ok(self.buffer.clone())),
            Err(e) => Some(Err(e)),
        }
    }
}

fn main() {
    let path = Path::new("./resources/zen_of_python.txt");
    let reader = FileLines::new(path).unwrap();
    for line in reader {
        print!("{}", line.unwrap());
    }
}
