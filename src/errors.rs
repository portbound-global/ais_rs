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

#[derive(Debug, Clone)]
pub enum AISError {
    MalformedSentence(String),
    TypeConversion { field: String, value: String },
    InvalidChecksum { expected: u8, found: u8 },
    IsMultipartSentence { current_part: u8, total_parts: u8 },
}

impl fmt::Display for AISError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AISError::MalformedSentence(s) => write!(f, "Malformed sentence: {}", s),
            AISError::TypeConversion { field, value } => {
                write!(f, "Failed to convert field {} with value {}", field, value)
            }
            AISError::InvalidChecksum { expected, found } => {
                write!(
                    f,
                    "Invalid checksum: expected {:X}, found {:X}",
                    expected, found
                )
            }
            AISError::IsMultipartSentence {
                current_part,
                total_parts,
            } => {
                write!(
                    f,
                    "Sentence is multipart: current part is {:X}, needs {:X} parts",
                    current_part, total_parts
                )
            }
        }
    }
}
