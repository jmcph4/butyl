pub enum Format {
    DOS,
    COFF,
    PE,
    Unknown
}

impl Format {
    pub fn from_string(string: String) -> Format {
        match string.as_ref() {
            "DOS" => Format::DOS,
            "COFF" => Format::COFF,
            "PE" => Format::PE,
            _ => Format::Unknown
        }
    }
}

