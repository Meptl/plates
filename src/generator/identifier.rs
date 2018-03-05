/// Various representations of a variable name.
///
/// Note that the use of the cached! macro is safe here because this structure is
/// immutable after initialization.
pub struct ScalarName {
    /// Any string representation of the variable
    pub canonical: String
}

/// Naively cache the first return value of the expression.
///
/// Note that this does not support functions that use the 'return' keyword
macro_rules! cached {
    (pub fn $name:ident (&$self:ident) -> $ret:ty $body:block) => {
        pub fn $name(&$self) -> &$ret {
            static mut cached: Option<$ret> = None;
            unsafe {
                if let Some(ref c) = cached {
                    return c;
                }
            }
            let val = $body;
            unsafe {
                cached = Some(val);
                cached.as_ref().unwrap()
            }
        }
    };
}

impl ScalarName {
    /// Returns the CamelCase representation of canonical
    cached! {
        pub fn camel_case(&self) -> String {
            let mut output = String::new();
            for token in self.tokens() {
                for (i, c) in token.chars().enumerate() {
                    match i {
                        0 => output.extend(c.to_uppercase()),
                        _ => output.extend(c.to_lowercase()),
                    }
                }
            }
            output
        }
    }

    /// Returns the lowerCamelCase representation of canonical
    cached! {
        pub fn lower_camel_case(&self) -> String {
            let mut output = String::new();
            for (i, token) in self.tokens().enumerate() {
                for (j, c) in token.chars().enumerate() {
                    match (i, j) {
                        (0, _) => output.extend(c.to_lowercase()),
                        (_, 0) => output.extend(c.to_uppercase()),
                        _ => output.extend(c.to_lowercase()),
                    }
                }
            }
            output
        }
    }

    /// Returns the snake_case representation of canonical
    cached! {
        pub fn snake_case(&self) -> String {
            self.tokens()
                .map(|s| s.to_lowercase())
                .intersperse(String::from("_"))
                .collect()
        }
    }

    /// Returns an iterator for the tokens of canonical
    pub fn tokens<'a>(&'a self) -> impl Iterator<Item = &'a str> {
        self.canonical.split_whitespace()
                      .flat_map(|w| w.split(&['_', '-'][..]))
                      .flat_map(|w| w.chars().split_camel_case())
    }
}

use std::str::Chars;

pub trait SplitCamelCaseExt<'a>: Iterator + Sized {
    fn split_camel_case(self) -> SplitCamelCase<'a>;
}

impl<'a> SplitCamelCaseExt<'a> for Chars<'a> {
    fn split_camel_case(self) -> SplitCamelCase<'a> {
        let value = self.as_str().clone();
        SplitCamelCase { iter: self, value: value, start: 0, end: 0 }
    }
}

/// Iterate over the tokens of a camelcase.
pub struct SplitCamelCase<'a> {
    iter: Chars<'a>,
    value: &'a str,
    start: usize,
    end: usize,
}

impl<'a> Iterator for SplitCamelCase<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        // Check for end and assume a token size of atleast 1.
        if let None = self.iter.next() {
            return None;
        }

        self.start = self.end;
        let token_len = 1 + self.iter.clone().take_while(|c| !c.is_uppercase()).count();
        self.end += token_len;

        // token_len includes an extra char, so we pop one less.
        for _ in 1..token_len {
            self.iter.next().unwrap();
        }

        Some(&self.value[self.start..self.end])
    }
}

