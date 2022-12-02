use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct InputString {
    inner: String,
}

impl InputString {
    pub fn parse<F: FromStr>(&self) -> F {
        use std::any::type_name;
        let Ok(f) = self.inner.parse::<F>() else {
        panic!("Could not parse \"{}\" to {}", self.inner, type_name::<F>());
    };
        f
    }

    pub fn try_parse<F: FromStr>(&self) -> Option<F> {
        self.inner.parse::<F>().ok()
    }

    pub fn split_n<const N: usize>(&self, p: &str) -> [InputString; N] {
        self.inner
            .split(p)
            .map(|s| InputString::from(s.to_string()))
            .take(N)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
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

impl PartialEq<&str> for InputString {
    fn eq(&self, other: &&str) -> bool {
        self.inner == *other
    }
}

impl PartialEq<InputString> for &str {
    fn eq(&self, other: &InputString) -> bool {
        *self == other.inner
    }
}

impl PartialEq<String> for InputString {
    fn eq(&self, other: &String) -> bool {
        self.inner == *other
    }
}

impl PartialEq<InputString> for String {
    fn eq(&self, other: &InputString) -> bool {
        *self == other.inner
    }
}
