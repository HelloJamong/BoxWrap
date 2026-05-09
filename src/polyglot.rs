const PNG_MAGIC: &[u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
const JPG_MAGIC: &[u8] = &[0xFF, 0xD8, 0xFF];
const ZIP_MAGIC: &[u8] = &[0x50, 0x4B, 0x03, 0x04];
const EOCD_SIG: &[u8] = &[0x50, 0x4B, 0x05, 0x06];
const CD_SIG: &[u8] = &[0x50, 0x4B, 0x01, 0x02];

pub fn validate_image(bytes: &[u8]) -> Result<(), WrapError> {
    if bytes.starts_with(PNG_MAGIC) || bytes.starts_with(JPG_MAGIC) {
        Ok(())
    } else {
        Err(WrapError::InvalidImage)
    }
}

pub fn validate_zip(bytes: &[u8]) -> Result<(), WrapError> {
    if bytes.starts_with(ZIP_MAGIC) {
        Ok(())
    } else {
        Err(WrapError::InvalidZip)
    }
}

pub fn wrap(image_bytes: &[u8], zip_bytes: &[u8]) -> Result<Vec<u8>, WrapError> {
    validate_image(image_bytes)?;
    validate_zip(zip_bytes)?;
    let adjusted = adjust_zip_offsets(zip_bytes, image_bytes.len() as u32)?;
    let mut out = Vec::with_capacity(image_bytes.len() + adjusted.len());
    out.extend_from_slice(image_bytes);
    out.extend_from_slice(&adjusted);
    Ok(out)
}

fn find_eocd(bytes: &[u8]) -> Option<usize> {
    if bytes.len() < 22 {
        return None;
    }
    // ZIP comment 최대 65535바이트이므로 끝에서 해당 범위만 역방향 탐색
    let search_start = bytes.len().saturating_sub(65535 + 22);
    bytes[search_start..]
        .windows(4)
        .rposition(|w| w == EOCD_SIG)
        .map(|pos| search_start + pos)
}

fn read_u16_le(bytes: &[u8], offset: usize) -> usize {
    u16::from_le_bytes([bytes[offset], bytes[offset + 1]]) as usize
}

fn read_u32_le(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]])
}

fn write_u32_le(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn adjust_zip_offsets(zip_bytes: &[u8], prefix_size: u32) -> Result<Vec<u8>, WrapError> {
    let eocd_pos = find_eocd(zip_bytes).ok_or(WrapError::InvalidZip)?;

    let cd_size = read_u32_le(zip_bytes, eocd_pos + 12) as usize;
    let cd_offset = read_u32_le(zip_bytes, eocd_pos + 16) as usize;

    if cd_offset + cd_size > zip_bytes.len() {
        return Err(WrapError::InvalidZip);
    }

    let mut result = zip_bytes.to_vec();

    // 각 Central Directory Entry의 local header offset 보정
    let mut pos = cd_offset;
    while pos + 46 <= cd_offset + cd_size {
        if &result[pos..pos + 4] != CD_SIG {
            break;
        }
        let filename_len = read_u16_le(&result, pos + 28);
        let extra_len = read_u16_le(&result, pos + 30);
        let comment_len = read_u16_le(&result, pos + 32);

        let current = read_u32_le(&result, pos + 42);
        write_u32_le(&mut result, pos + 42, current + prefix_size);

        pos += 46 + filename_len + extra_len + comment_len;
    }

    // EOCD의 Central Directory 시작 오프셋 보정
    let current = read_u32_le(&result, eocd_pos + 16);
    write_u32_le(&mut result, eocd_pos + 16, current + prefix_size);

    Ok(result)
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

#[cfg(test)]
mod tests {
    use super::*;

    // 파일 하나("a")를 포함한 최소 유효 ZIP 바이트 생성
    // 구조: [local file header 31B] [CD entry 47B] [EOCD 22B] = 100B
    fn minimal_zip() -> Vec<u8> {
        let mut zip = Vec::new();

        // Local file header (offset 0, 31 bytes)
        zip.extend_from_slice(&[0x50, 0x4B, 0x03, 0x04]); // signature
        zip.extend_from_slice(&[0x0A, 0x00]); // version needed
        zip.extend_from_slice(&[0x00, 0x00]); // flags
        zip.extend_from_slice(&[0x00, 0x00]); // compression: stored
        zip.extend_from_slice(&[0x00, 0x00]); // mod time
        zip.extend_from_slice(&[0x00, 0x00]); // mod date
        zip.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC-32
        zip.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // compressed size
        zip.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // uncompressed size
        zip.extend_from_slice(&[0x01, 0x00]); // filename length = 1
        zip.extend_from_slice(&[0x00, 0x00]); // extra length = 0
        zip.push(b'a'); // filename

        let cd_offset = zip.len() as u32; // = 31

        // Central Directory Entry (offset 31, 47 bytes)
        zip.extend_from_slice(&[0x50, 0x4B, 0x01, 0x02]); // signature
        zip.extend_from_slice(&[0x00, 0x00]); // version made by
        zip.extend_from_slice(&[0x0A, 0x00]); // version needed
        zip.extend_from_slice(&[0x00, 0x00]); // flags
        zip.extend_from_slice(&[0x00, 0x00]); // compression
        zip.extend_from_slice(&[0x00, 0x00]); // mod time
        zip.extend_from_slice(&[0x00, 0x00]); // mod date
        zip.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC-32
        zip.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // compressed size
        zip.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // uncompressed size
        zip.extend_from_slice(&[0x01, 0x00]); // filename length = 1
        zip.extend_from_slice(&[0x00, 0x00]); // extra length = 0
        zip.extend_from_slice(&[0x00, 0x00]); // comment length = 0
        zip.extend_from_slice(&[0x00, 0x00]); // disk number start
        zip.extend_from_slice(&[0x00, 0x00]); // internal attrs
        zip.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // external attrs
        zip.extend_from_slice(&0u32.to_le_bytes()); // local header offset = 0
        zip.push(b'a'); // filename

        let cd_size = (zip.len() as u32) - cd_offset; // = 47

        // EOCD (offset 78, 22 bytes)
        zip.extend_from_slice(&[0x50, 0x4B, 0x05, 0x06]); // signature
        zip.extend_from_slice(&[0x00, 0x00]); // disk number
        zip.extend_from_slice(&[0x00, 0x00]); // disk with CD
        zip.extend_from_slice(&[0x01, 0x00]); // entries on disk = 1
        zip.extend_from_slice(&[0x01, 0x00]); // total entries = 1
        zip.extend_from_slice(&cd_size.to_le_bytes()); // CD size = 47
        zip.extend_from_slice(&cd_offset.to_le_bytes()); // CD offset = 31
        zip.extend_from_slice(&[0x00, 0x00]); // comment length = 0

        zip // total 100 bytes
    }

    #[test]
    fn validate_image_accepts_png() {
        let bytes = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00];
        assert!(validate_image(bytes).is_ok());
    }

    #[test]
    fn validate_image_accepts_jpg() {
        let bytes = &[0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10];
        assert!(validate_image(bytes).is_ok());
    }

    #[test]
    fn validate_image_rejects_invalid() {
        assert!(matches!(
            validate_image(b"not an image"),
            Err(WrapError::InvalidImage)
        ));
    }

    #[test]
    fn validate_image_rejects_empty() {
        assert!(matches!(validate_image(&[]), Err(WrapError::InvalidImage)));
    }

    #[test]
    fn validate_zip_accepts_valid() {
        let bytes = &[0x50, 0x4B, 0x03, 0x04, 0x00, 0x00];
        assert!(validate_zip(bytes).is_ok());
    }

    #[test]
    fn validate_zip_rejects_invalid() {
        assert!(matches!(
            validate_zip(b"not a zip"),
            Err(WrapError::InvalidZip)
        ));
    }

    #[test]
    fn validate_zip_rejects_empty() {
        assert!(matches!(validate_zip(&[]), Err(WrapError::InvalidZip)));
    }

    #[test]
    fn find_eocd_locates_signature() {
        let zip = minimal_zip();
        // EOCD: 31 + 47 = offset 78
        assert_eq!(find_eocd(&zip), Some(78));
    }

    #[test]
    fn find_eocd_returns_none_on_short_input() {
        assert_eq!(find_eocd(&[0x00; 10]), None);
    }

    #[test]
    fn adjust_zip_offsets_increments_eocd_and_cd_entry() {
        let zip = minimal_zip();
        let prefix = 100u32;
        let adjusted = adjust_zip_offsets(&zip, prefix).unwrap();

        let eocd_pos = find_eocd(&adjusted).unwrap();

        // EOCD CD offset: 31 + 100 = 131
        assert_eq!(read_u32_le(&adjusted, eocd_pos + 16), 131);

        // CD entry local header offset (at CD start + 42 = 31 + 42 = 73): 0 + 100 = 100
        assert_eq!(read_u32_le(&adjusted, 31 + 42), 100);
    }

    #[test]
    fn adjust_zip_offsets_rejects_invalid() {
        assert!(adjust_zip_offsets(b"not a zip with eocd padding____", 100).is_err());
    }

    #[test]
    fn wrap_output_starts_with_image_magic() {
        let png = {
            let mut v = PNG_MAGIC.to_vec();
            v.extend_from_slice(&[0u8; 8]);
            v
        };
        let zip = minimal_zip();
        let result = wrap(&png, &zip).unwrap();
        assert!(result.starts_with(PNG_MAGIC));
    }

    #[test]
    fn wrap_rejects_invalid_image() {
        let zip = minimal_zip();
        assert!(matches!(
            wrap(b"not an image", &zip),
            Err(WrapError::InvalidImage)
        ));
    }

    #[test]
    fn wrap_rejects_invalid_zip() {
        let png = PNG_MAGIC.to_vec();
        assert!(matches!(
            wrap(&png, b"not a zip"),
            Err(WrapError::InvalidZip)
        ));
    }
}
