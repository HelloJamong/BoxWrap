pub fn wrap(_image_bytes: &[u8], _zip_bytes: &[u8]) -> Result<Vec<u8>, WrapError> {
    todo!()
}

#[derive(Debug, thiserror::Error)]
pub enum WrapError {
    #[error("유효하지 않은 이미지 파일입니다")]
    InvalidImage,
    #[error("유효하지 않은 ZIP 파일입니다")]
    InvalidZip,
    #[error("IO 오류: {0}")]
    Io(#[from] std::io::Error),
}
