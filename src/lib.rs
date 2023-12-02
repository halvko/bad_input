//! A library for input parsing with panicy error handeling, and an unusual amout of string cloning.
//!
//! To get started, give anything implementing std::io::Read to [BadInput::new()] (e.g.
//! [stdin](std::io::stdin) or a byte slice), and be prepared for your program to crash as soon as the input
//! does not conform to your expectations :)
//!
//! ```
//! let s = "Very,8;fancy,82;string,11";
//!
//! let mut input = bad_input::BadInput::new(s.as_bytes());
//!
//! let [w1, n1, w2, n2, w3, n3] = input.line().destruct_n([",", ";"]);
//!
//! assert_eq!(format!("{w1} {w2} {w3}"), "Very fancy string");
//!
//! assert_eq!(n1.parse::<u64>() + n2.parse::<u64>() + n3.parse::<u64>(), 101);
//! ```

use std::{io::Read, string::FromUtf8Error};

pub use input_string::InputString;

pub struct BadInput<R: Read> {
    reader: R,
    read_buf: [u8; 1024],
    buf: Vec<u8>,
}

impl<R: Read> BadInput<R> {
    /// Creates a new BadInput from any reader.
    ///
    /// # Examples
    ///
    /// Taking input from stdin:
    /// ```
    /// use std::io::stdin;
    ///
    /// use bad_input::BadInput;
    /// let mut input = BadInput::new(stdin());
    /// ```
    ///
    /// Taking input from some other reader:
    /// ```
    /// use bad_input::BadInput;
    ///
    /// let mut input = BadInput::new("Hello, world!\nGood bye!".as_bytes());
    /// assert_eq!(input.line(), "Hello, world!");
    /// assert_eq!(input.line(), "Good bye!");
    /// ```
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            read_buf: [0; 1024],
            buf: Vec::new(),
        }
    }

    /// Reads a line from the input
    ///
    /// # Examples
    /// ```
    /// use bad_input::BadInput;
    ///
    /// let mut input = BadInput::new("What is\ngoing on!?".as_bytes());
    /// assert_eq!(input.line(), "What is");
    /// assert_eq!(input.line(), "going on!?")
    /// ```
    ///
    /// # Panics
    ///
    /// - If the line is not valid UTF-8 or
    /// - If there are no more to be read.
    ///
    /// See [try_line](BadInput::try_line) for an alternative which only panics on invalid UTF-8
    pub fn line(&mut self) -> InputString {
        self.try_line().unwrap()
    }

    /// Creates an iterator over lines from the input. Note that only the lines pulled from the
    /// iterator is removed from the input. The iterator will panic if invalid UTF-8 is encountered.
    ///
    /// # Examples
    /// ```
    /// use bad_input::BadInput;
    ///
    /// let mut input = BadInput::new("Here are multiple\nlines.".as_bytes());
    /// let mut lines = input.lines();
    /// for line in lines {
    ///     // do something with the line
    /// }
    /// assert_eq!(input.try_line(), None)
    /// ```
    ///
    /// ```
    /// use bad_input::BadInput;
    ///
    /// let mut input = BadInput::new("Here are multiple\nlines.".as_bytes());
    /// let mut lines = input.lines();
    /// for line in lines {
    ///     assert_eq!(line, "Here are multiple");
    ///     break;
    /// }
    /// assert_eq!(input.line(), "lines.")
    /// ```
    pub fn lines<'a>(&'a mut self) -> impl Iterator<Item = InputString> + 'a {
        Lines { input: self }
    }

    /// Reads the next line from the input, returning `None` if the input is empty.
    ///
    /// # Examples
    /// ```
    /// use bad_input::BadInput;
    ///
    /// let mut input = BadInput::new("Two\nlines".as_bytes());
    /// assert_eq!(input.try_line(), Some("Two".into()));
    /// assert_eq!(input.try_line(), Some("lines".into()));
    /// assert_eq!(input.try_line(), None);
    /// ```
    ///
    /// # Panics
    ///
    /// If the line contains invalid UTF-8
    pub fn try_line(&mut self) -> Option<InputString> {
        self.try_read_to_byte(b'\n')
            .or_else(|| self.empty_buffer().map(|r| r.map_err(|e| e.into())))
            .and_then(|e| e.ok())
            .map(|line| {
                let Some(line) = line.strip_suffix('\n') else {
                    return line.into();
                };
                let Some(line) = line.strip_suffix('\r') else {
                    return line.into();
                };
                line.into()
            })
    }

    fn empty_buffer(&mut self) -> Option<Result<String, FromUtf8Error>> {
        if self.buf.is_empty() {
            return None;
        }
        Some(String::from_utf8(std::mem::take(&mut self.buf)))
    }

    fn try_read_to_byte(&mut self, p: u8) -> Option<Result<String, ReadToCharError>> {
        let r = &mut self.reader;

        let mut old_buf = std::mem::take(&mut self.buf);

        // First check if we already had the character in our buffer
        for (i, c) in old_buf.iter().copied().enumerate() {
            if c == p {
                self.buf.extend_from_slice(&old_buf[(i + 1)..]);
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

mod input_string;

struct Lines<'a, R: Read> {
    input: &'a mut BadInput<R>,
}

impl<'a, R: Read> Iterator for Lines<'a, R> {
    type Item = InputString;

    fn next(&mut self) -> Option<Self::Item> {
        self.input.try_line()
    }
}
