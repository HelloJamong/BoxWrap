use boxwrap::polyglot;
use std::io::{Cursor, Write};

const PNG_MAGIC: &[u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

fn make_png() -> Vec<u8> {
    let mut v = PNG_MAGIC.to_vec();
    v.extend_from_slice(&[0u8; 16]);
    v
}

fn make_zip() -> Vec<u8> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(cursor);
    let options =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    writer.start_file("hello.txt", options).unwrap();
    writer.write_all(b"hello").unwrap();
    writer.finish().unwrap().into_inner()
}

#[test]
fn wrap_output_starts_with_png_magic() {
    let result = polyglot::wrap(&make_png(), &make_zip()).unwrap();
    assert!(result.starts_with(PNG_MAGIC));
}

#[test]
fn wrap_output_is_valid_zip() {
    let result = polyglot::wrap(&make_png(), &make_zip()).unwrap();
    let archive =
        zip::ZipArchive::new(Cursor::new(result)).expect("포장된 파일이 유효한 ZIP이어야 합니다");
    assert_eq!(archive.len(), 1);
}

#[test]
fn wrap_rejects_non_image() {
    let result = polyglot::wrap(b"not an image", &make_zip());
    assert!(matches!(result, Err(polyglot::WrapError::InvalidImage)));
}

#[test]
fn wrap_rejects_non_zip() {
    let result = polyglot::wrap(&make_png(), b"not a zip");
    assert!(matches!(result, Err(polyglot::WrapError::InvalidZip)));
}
