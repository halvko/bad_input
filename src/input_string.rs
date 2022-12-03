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

    pub fn split_at(&self, n: usize) -> (InputString, InputString) {
        let (lhs, rhs) = self.inner.split_at(n);
        (lhs.to_string().into(), rhs.to_string().into())
    }

    pub fn chars<'a>(&'a self) -> impl Iterator<Item = char> + 'a {
        self.inner.chars()
    }

    pub fn destruct<const N: usize, const M: usize>(
        &self,
        splitters: [&str; N],
    ) -> [InputString; M] {
        let mut res = Vec::new();
        let mut rest = self.as_str();
        'outer: loop {
            for s in &splitters {
                if res.len() == M {
                    break 'outer;
                }
                let (part, next) = rest.split_once(s).unwrap();
                res.push(part.to_string().into());
                rest = next;
            }
        }
        res.try_into().unwrap()
    }

    pub fn as_str(&self) -> &str {
        &self.inner
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

impl std::fmt::Display for InputString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
