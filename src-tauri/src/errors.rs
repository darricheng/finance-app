// Create errors that implements serialize
// See: https://tauri.app/v1/guides/features/command/#error-handling
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error(transparent)]
    CSV {
        #[from]
        source: csv::Error,
    },
    #[error("Error parsing date, reason: {:?}", reason)]
    Date { reason: String },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
