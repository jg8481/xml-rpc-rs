use serde::de;
use std::fmt::Display;

error_chain!{
    errors {
        Decoding(t: String) {
            description("Issue while decoding data structure")
            display("Issue while decoding data structure: {}", t)
        }
        UnsupportedData(t: String) {
            description("Given structure is not supported")
            display("Given structure is not supported: {}", t)
        }
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        ErrorKind::Decoding(format!("{}", msg)).into()
    }

    fn invalid_type(unexp: de::Unexpected, exp: &de::Expected) -> Self {
        if let de::Unexpected::Unit = unexp {
            Error::custom(format_args!("invalid type: null, expected {}", exp))
        } else {
            Error::custom(format_args!("invalid type: {}, expected {}", unexp, exp))
        }
    }
}