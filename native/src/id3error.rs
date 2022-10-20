#[derive(Debug)]
pub struct ID3Error(pub String);

impl std::fmt::Display for ID3Error
{
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "There was an error with id3tags:\n {}", self.0)
    }
}

impl std::error::Error for ID3Error {}