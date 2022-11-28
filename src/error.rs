use snafu::Snafu;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
#[snafu(context(suffix(false)))]
pub enum Error {
    #[snafu(display("http error: {source}"))]
    Http { source: reqwest::Error },
    #[snafu(display("unable to build header: {source}"))]
    HeaderParse {
        source: reqwest::header::InvalidHeaderValue,
    },
    #[snafu(display("unexpected response status: {status}, text: {text:?}"))]
    UnexpectedStatus {
        status: reqwest::StatusCode,
        text: String,
    },
    #[snafu(display("unexpected json: {source}"))]
    UnexpectedJson { source: serde_json::Error },
}
// TODO: parse error info
