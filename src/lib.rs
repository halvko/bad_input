use std::{any::type_name, io::Read, str::FromStr, string::FromUtf8Error};

pub struct BadInput<R: Read> {
    reader: R,
    read_buf: [u8; 1024],
    buf: Vec<u8>,
}

impl<R: Read> BadInput<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            read_buf: [0; 1024],
            buf: Vec::new(),
        }
    }

    pub fn input(&mut self) -> InputString {
        self.try_read_line().unwrap()
    }

    pub fn try_read_line(&mut self) -> Option<InputString> {
        self.try_read_to_byte(b'\n')
            .and_then(|e| e.ok())
            .map(|v| String::from_utf8(v).unwrap().into())
    }

    fn try_read_to_byte(&mut self, p: u8) -> Option<Result<Vec<u8>, ReadToCharError>> {
        let r = &mut self.reader;

        let mut old_buf = std::mem::take(&mut self.buf);

        // First check if we already had the character in our buffer
        for (i, c) in old_buf.iter().copied().enumerate() {
            if c == p {
                self.buf.extend_from_slice(&old_buf[i..]);
                old_buf.truncate(i);
                return Some(
                    String::from_utf8(old_buf)
                        .map(|s| s.into())
                        .map_err(|e| e.into()),
                );
            }
        }

        // We didn't so we have to try to read some characters
        loop {
            match r.read(&mut self.read_buf) {
                Ok(bytes) => {
                    if bytes == 0 {
                        self.buf = old_buf;
                        return None;
                    }
                    let read = &self.read_buf[..bytes];
                    for (i, b) in read.iter().copied().enumerate() {
                        if b == p {
                            self.buf.extend_from_slice(&read[(i + 1)..]);
                            return Some(
                                String::from_utf8(old_buf)
                                    .map(|s| s.into())
                                    .map_err(|e| e.into()),
                            );
                        }
                        old_buf.push(b);
                    }
                }
                Err(e) => {
                    if let std::io::ErrorKind::Interrupted = e.kind() {
                        continue;
                    }
                    return Some(Err(e.into()));
                }
            }
        }
    }
}

enum ReadToCharError {
    InvalidUtf8(FromUtf8Error),
    IoError(std::io::Error),
}

impl From<std::io::Error> for ReadToCharError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<FromUtf8Error> for ReadToCharError {
    fn from(e: FromUtf8Error) -> Self {
        Self::InvalidUtf8(e)
    }
}

#[derive(Debug)]
pub struct InputString {
    inner: String,
}

impl InputString {
    pub fn parse<F: FromStr>(&self) -> F {
        let Ok(f) = self.inner.parse::<F>() else {
            panic!("Could not parse \"{}\" to {}", self.inner, type_name::<F>());
        };
        f
    }

    pub fn try_parse<F: FromStr>(&self) -> Option<F> {
        self.inner.parse::<F>().ok()
    }
}

impl Into<String> for InputString {
    fn into(self) -> String {
        self.inner
    }
}

impl From<String> for InputString {
    fn from(inner: String) -> Self {
        Self { inner }
    }
}
