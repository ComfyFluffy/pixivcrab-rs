use snafu::Snafu;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
#[snafu(context(suffix(false)))]
pub enum Error {
    #[snafu(display("http error occurred: {source}"))]
    Http { source: reqwest::Error },
    #[snafu(display("unable to build header: {source}"))]
    HeaderParse {
        source: reqwest::header::InvalidHeaderValue,
    },
    #[snafu(display("got unexpected response status: {status}, text: {text}"))]
    UnexpectedStatus {
        status: reqwest::StatusCode,
        text: String,
    },
}
// TODO: parse error info
