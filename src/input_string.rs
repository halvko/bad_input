use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct InputString {
    inner: String,
}

impl InputString {
    pub fn new() -> Self {
        Self {
            inner: String::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

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

    pub fn split<'a>(&'a self, p: &'a str) -> impl Iterator<Item = Self> + 'a {
        self.inner.split(p).map(|s| s.into())
    }

    /*
    fn split_with(
        &'a self,
        mut splitter: impl FnMut(&str) -> Option<(&str, &str)>,
    ) -> impl Iterator<Item = Self> + 'a {
        let mut rest = self.as_str();
        std::iter::repeat_with(move || {})
    } */

    /// Returns an array of `N` [InputString]s, with the result from `N` times splitting the input
    /// by `p`.
    ///
    /// # Examples
    /// ```
    /// use bad_input::InputString;
    ///
    /// let input = InputString::from("ra la lu li");
    /// let [fst, snd, rest] = input.split_n(" ");
    /// assert_eq!(fst, "ra");
    /// assert_eq!(snd, "la");
    /// assert_eq!(rest, "lu li");
    /// ```
    pub fn split_n<const N: usize>(&self, p: &str) -> [InputString; N] {
        self.destruct_n([p])
    }

    pub fn try_split_n<const N: usize>(&self, p: &str) -> Option<[InputString; N]> {
        self.try_destruct_n([p])
    }

    pub fn split_at(&self, n: usize) -> (InputString, InputString) {
        let (lhs, rhs) = self.inner.split_at(n);
        (lhs.to_string().into(), rhs.to_string().into())
    }

    pub fn chars<'a>(&'a self) -> impl Iterator<Item = char> + 'a {
        self.inner.chars()
    }

    /// Splits the string by the `N` given splitters, first finding the substring before the first,
    /// then the one between the first and the second and so on. If the output is not yet filled
    /// after the last splitter is applied, the first one is reapplied after it, and all of the
    /// splitters are applied again to the rest of the input. Destruct always places the rest of
    /// the input in the last variable.
    ///
    /// # Examples
    ///
    /// ```
    /// use bad_input::InputString;
    ///
    /// let input: InputString = "a,b;c,d;e,f".into();
    /// let [a, b, c, d, rest] = input.destruct_n([",", ";"]);
    /// assert_eq!([a, b, c, d, rest], ["a", "b", "c", "d", "e,f"])
    /// ```
    ///
    /// # Panics
    ///
    /// If one of the splitters could not be applied
    pub fn destruct_n<const N: usize, const M: usize>(
        &self,
        splitters: [&str; N],
    ) -> [InputString; M] {
        if splitters.is_empty() {
            panic!("At least one splitter is neccesary")
        }
        let mut case = 0;
        self.destruct_n_with(move |s| {
            let res = splitters[case];
            case = (case + 1) % N;
            s.split_once(res).unwrap()
        })
    }

    fn try_destruct_n<const N: usize, const M: usize>(
        &self,
        splitters: [&str; N],
    ) -> Option<[InputString; M]> {
        if splitters.is_empty() {
            panic!("At least one splitter is neccesary")
        }
        let mut case = 0;
        self.try_destruct_n_with(move |s| {
            let res = splitters[case];
            case = (case + 1) % N;
            s.split_once(res)
        })
    }

    fn destruct_n_with<const M: usize>(
        &self,
        mut splitter: impl FnMut(&str) -> (&str, &str),
    ) -> [Self; M] {
        let mut res = Vec::new();
        let mut rest = self.as_str();
        loop {
            if res.len() == (M - 1) {
                res.push(rest.into());
                break res.try_into().unwrap();
            }
            let (part, next) = splitter(rest);
            res.push(part.into());
            rest = next
        }
    }

    fn try_destruct_n_with<const M: usize>(
        &self,
        mut splitter: impl FnMut(&str) -> Option<(&str, &str)>,
    ) -> Option<[Self; M]> {
        let mut res = Vec::new();
        let mut rest = self.as_str();
        loop {
            if res.len() == (M - 1) {
                res.push(rest.into());
                break Some(res.try_into().unwrap());
            }
            let (part, next) = splitter(rest)?;
            res.push(part.into());
            rest = next
        }
    }

    pub fn is_empty(&self) -> bool {
        self.as_str() == ""
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }

    pub fn bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    pub fn trim(&self) -> Self {
        self.inner.trim().into()
    }
}

impl<D: std::fmt::Display> std::ops::Add<D> for InputString {
    type Output = InputString;

    fn add(self, rhs: D) -> Self::Output {
        format!("{self}{rhs}").into()
    }
}

impl<D: std::fmt::Display> std::ops::AddAssign<D> for InputString {
    fn add_assign(&mut self, rhs: D) {
        self.inner += format!("{rhs}").as_str()
    }
}

impl<Addable> FromIterator<Addable> for InputString
where
    InputString: std::ops::AddAssign<Addable>,
{
    fn from_iter<T: IntoIterator<Item = Addable>>(iter: T) -> Self {
        let mut res = InputString::new();
        for s in iter {
            res += s
        }
        res
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

impl From<&str> for InputString {
    fn from(inner: &str) -> Self {
        Self {
            inner: inner.to_owned(),
        }
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
