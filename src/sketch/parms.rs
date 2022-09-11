use std::{env::Args, fmt, iter::Skip, mem, ops::RangeInclusive, str::FromStr};

pub trait CliArg {
    // An error is a panic when parsing
    fn parse_args(&mut self, args: &mut CliTokens);
    fn schema(&self) -> Schema;
}

pub enum Schema {
    String(String),
    Int(i128, RangeInclusive<i128>),
    Double(f64, RangeInclusive<f64>),
    Bool(bool),
    Usize(usize),
    Isize(isize),
}

pub struct CliTokens {
    value: String,
    args: Skip<Args>,
}

#[macro_export]
macro_rules! sketch_parms {
    ($($name: ident: $t: ty = $def: expr),* $(,)?) => {
        use std::io::{self, Write};

        #[derive(Debug)]
        struct Parms {
            $(pub $name: $t),*
        }

        impl Default for Parms {
            fn default() -> Self {
                Parms {
                    $(
                        $name: $def,
                    )*
                }
            }
        }

        impl Parms {
            pub fn from_cli() -> Self {
                let mut args = CliTokens::from_env();

                let mut out = Parms::default();

                while let Some(option) = args.next() {
                    let option_name = match option.strip_prefix("--") {
                        None => {
                            // only long options are supported or unparsed value
                            continue;
                        }
                        Some(o) => o,
                    };

                    $(
                        if option_name == stringify!($name) {
                            out.$name.parse_args(&mut args);
                        }
                    )*
                }

                skv_log!("MANIFEST", Parms::default().manifest());

                out
            }

            pub fn manifest(&self) -> String {
                use std::fmt::Write;

                let mut s = "{".to_string();

                let mut pop_coma = false;

                $(
                    write!(s, r#""{}":"#, stringify!($name)).unwrap();
                    self.$name.schema().dump(&mut s).unwrap();
                    s.push(',');
                    pop_coma = true;
                )*

                // thank you json
                if pop_coma {
                    s.pop();
                }

                s.push('}');
                s
            }
        }
    };
}

impl CliTokens {
    pub fn from_env() -> Self {
        Self {
            value: String::new(),
            args: std::env::args().skip(1),
        }
    }

    fn parsed_next<T: FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        self.next().unwrap().parse().unwrap()
    }
}

impl Iterator for CliTokens {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.value.is_empty() {
            return Some(mem::take(&mut self.value));
        }

        let mut option = self.args.next()?;

        if let Some(i) = option.find('=') {
            self.value = option.split_off(i);
            self.value.remove(0);
        }

        Some(option)
    }
}

impl Schema {
    pub fn dump<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        match self {
            Schema::String(ss) => {
                write!(out, r#"{{"type": "string", "default": "{ss}"}}"#)
            }
            Schema::Int(i, range) => {
                write!(
                    out,
                    r#"{{"type": "int", "default": {i}, "min": {}, "max": {}}}"#,
                    range.start(),
                    range.end()
                )
            }
            Schema::Double(d, range) => {
                write!(
                    out,
                    r#"{{"type": "double", "default": {d}, "min": {}, "max": {}}}"#,
                    range.start(),
                    range.end()
                )
            }
            Schema::Bool(b) => {
                write!(out, r#"{{"type": "bool", "default": {b}}}"#)
            }
            Schema::Isize(i) => {
                write!(
                    out,
                    r#"{{"type": "int", "default": {i}, "min": {}, "max": {}}}"#,
                    isize::MIN,
                    isize::MAX
                )
            }
            Schema::Usize(i) => {
                write!(
                    out,
                    r#"{{"type": "int", "default": {i}, "min": {}, "max": {}}}"#,
                    usize::MIN,
                    usize::MAX
                )
            }
        }
    }
}

impl CliArg for bool {
    fn parse_args(&mut self, args: &mut CliTokens) {
        let n = args.next().unwrap().to_lowercase();
        if n == "true" || n == "1" {
            *self = true;
        } else if n == "false" || n == "0" {
            *self = false;
        } else {
            panic!("{n} is a valid bool, note that even boolean flags require an explicit value");
        }
    }

    fn schema(&self) -> Schema {
        Schema::Bool(*self)
    }
}

impl CliArg for String {
    fn parse_args(&mut self, args: &mut CliTokens) {
        *self = args.parsed_next();
    }

    fn schema(&self) -> Schema {
        Schema::String(self.clone())
    }
}

macro_rules! impl_num_arg {
    ($t: ty, $tt: ty, $variant: ident) => {
        impl CliArg for $t {
            fn parse_args(&mut self, args: &mut CliTokens) {
                *self = args.parsed_next();
            }

            fn schema(&self) -> Schema {
                Schema::$variant(
                    <$tt>::from(*self),
                    <$tt>::from(<$t>::MIN)..=<$tt>::from(<$t>::MAX),
                )
            }
        }
    };
}

impl_num_arg!(u8, i128, Int);
impl_num_arg!(u16, i128, Int);
impl_num_arg!(u32, i128, Int);
impl_num_arg!(u64, i128, Int);
impl_num_arg!(i8, i128, Int);
impl_num_arg!(i16, i128, Int);
impl_num_arg!(i32, i128, Int);
impl_num_arg!(i64, i128, Int);

impl_num_arg!(f32, f64, Double);
impl_num_arg!(f64, f64, Double);

impl CliArg for usize {
    fn parse_args(&mut self, args: &mut CliTokens) {
        *self = args.parsed_next();
    }

    fn schema(&self) -> Schema {
        Schema::Usize(*self)
    }
}

impl CliArg for isize {
    fn parse_args(&mut self, args: &mut CliTokens) {
        *self = args.parsed_next();
    }

    fn schema(&self) -> Schema {
        Schema::Isize(*self)
    }
}
