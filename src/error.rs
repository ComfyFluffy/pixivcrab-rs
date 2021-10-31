use snafu::Snafu;

#[derive(Snafu, Debug)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("HTTP error occurred: {}", source))]
    HTTP { source: reqwest::Error },
    #[snafu(display("Unable to build header: {}", source))]
    HeaderParse {
        source: reqwest::header::InvalidHeaderValue,
    },
}
