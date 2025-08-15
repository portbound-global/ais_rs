use std::fmt;

#[macro_export]
macro_rules! log_error_sentence {
    ($err:expr, $msg:expr) => {
        println!(
            "{}\n\x1b[31m|     problem sentence: {}\x1b[0m\n",
            $err, $msg
        );
    };
}

pub enum AisError {
    MalformedSentence(String),
    TypeConversion { field: String, value: String },
    InvalidChecksum { expected: u8, found: u8 },
}

impl fmt::Display for AisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AisError::MalformedSentence(s) => write!(f, "Malformed sentence: {}", s),
            AisError::TypeConversion { field, value } => {
                write!(f, "Failed to convert field {} with value {}", field, value)
            }
            AisError::InvalidChecksum { expected, found } => {
                write!(
                    f,
                    "Invalid checksum: expected {:X}, found {:X}",
                    expected, found
                )
            }
        }
    }
}
