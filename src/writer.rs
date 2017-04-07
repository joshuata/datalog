use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::Error;
use predicates::Stmt;

pub trait Writer {
    fn add(&mut self, Stmt) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct FileWriter {
    file: File,
}

impl FileWriter {
    pub fn new(path: &Path) -> FileWriter {
        FileWriter { file: File::create(path).unwrap() }
    }
}

impl Writer for FileWriter {
    fn add(&mut self, statement: Stmt) -> Result<(), Error> {
        match statement {
            Stmt::Fact(pred) => writeln!(self.file, "{}.", pred),
            Stmt::Rule(head, tail) => {
                write!(self.file, "{} :- ", head).unwrap();
                tail.iter().fold(true, |first, elem| {
                    if !first {
                        write!(self.file, ", ").unwrap();
                    }
                    write!(self.file, "{}", elem).unwrap();
                    false
                });
                writeln!(self.file, ".")
            }
            Stmt::Query(pred) => writeln!(self.file, "{}?", pred),
        }
    }
}

impl Drop for FileWriter {
    fn drop(&mut self) {
        self.file.flush().unwrap();
    }
}
