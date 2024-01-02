pub mod video {
    use std::{error::Error, fmt};
    pub type VideoResult<T> = Result<T, VideoError>;

    #[derive(Debug)]
    pub enum VideoError {
        ContainerDecodingError,
        VideoDecodingError,
    }

    impl Error for VideoError {}

    impl fmt::Display for VideoError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::ContainerDecodingError => {
                    write!(f, "Was unable to decode the container properly.")
                }
                Self::VideoDecodingError => {
                    write!(f, "Was unable to decode the video track properly.")
                }
            }
        }
    }
}
