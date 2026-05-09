# BoxWrap

ZIP 압축 파일을 PNG/JPG 이미지 안에 숨겨 공유하는 Windows GUI 애플리케이션입니다.

[![GitHub Release](https://img.shields.io/github/v/release/HelloJamong/BoxWrap)](https://github.com/HelloJamong/BoxWrap/releases)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

## 주요 기능

- **ZIP 포장**: PNG/JPG 이미지 안에 ZIP 파일을 삽입하여 단일 파일로 출력
- **이미지 유지**: 포장된 파일은 이미지 뷰어에서 원본 이미지 그대로 표시
- **압축 해제**: 출력 파일의 확장자를 `.zip`으로 변경하면 7-Zip, Windows 탐색기로 압축 해제 가능

## 기술 스택

- **언어**: Rust
- **GUI**: egui + eframe
- **빌드/배포**: GitHub Actions (Windows 네이티브 빌드)

## 사용 방법

1. [최신 릴리즈](https://github.com/HelloJamong/BoxWrap/releases)에서 `BoxWrap.exe` 다운로드
2. 실행 후 포장지 이미지(PNG/JPG)와 압축 파일(ZIP) 선택
3. **포장하기** 버튼 클릭 → 출력 파일 저장
4. 수신자는 출력 파일 확장자를 `.zip`으로 변경하여 압축 해제
