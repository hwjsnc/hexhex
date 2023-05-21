#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Case {
    #[default]
    Lower,
    Upper,
}

/// Options for displaying bytes as hex
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DisplayOptions {
    /// Whether '0x' (sans quotes) should be prefixed.
    /// Note that the case option is ignored for the prefix.
    pub with_prefix: bool,
    /// Upper or lower case letters (A-F or a-f).
    pub case: Case,
}

/// Wrapper struct to display bytes as hex
#[derive(Debug, Clone, Copy)]
pub struct Hex<T> {
    options: DisplayOptions,
    data: T,
}

impl<T: AsRef<[u8]>> core::fmt::Display for Hex<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.options.with_prefix {
            write!(f, "0x")?;
        }
        match self.options.case {
            Case::Lower => {
                for byte in self.data.as_ref() {
                    write!(f, "{byte:02x}")?;
                }
            }
            Case::Upper => {
                for byte in self.data.as_ref() {
                    write!(f, "{byte:02X}")?;
                }
            }
        }
        Ok(())
    }
}

impl<T: AsRef<[u8]>> Hex<T> {
    /// Create a new wrapper struct to display the content of data as hex with default display options.
    pub fn new(data: T) -> Self {
        Self::new_with_options(data, DisplayOptions::default())
    }

    /// Create a new wrapper struct to display the content of data as hex with the given options.
    pub fn new_with_options(data: T, options: DisplayOptions) -> Self {
        Self { options, data }
    }
}

impl<T> Hex<T> {
    /// Use the given display options
    pub fn with_options(mut self, options: DisplayOptions) -> Self {
        self.options = options;
        self
    }

    /// Display with or without prefix
    pub fn with_prefix(mut self, with_prefix: bool) -> Self {
        self.options.with_prefix = with_prefix;
        self
    }

    /// Display in lower or upper case
    pub fn with_case(mut self, case: Case) -> Self {
        self.options.case = case;
        self
    }
}
