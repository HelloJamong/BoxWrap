# Changelog

## [26.1.0] - 2026-05-09

### Added
- ZIP Polyglot 포장 기능
  - PNG/JPG 이미지 안에 ZIP 파일을 삽입하는 핵심 알고리즘 구현
  - EOCD 및 Central Directory Entry 오프셋 보정으로 7-Zip, Windows 탐색기 모두 정상 압축 해제 지원
  - 이미지 뷰어에서는 원본 이미지 그대로 표시
- Windows GUI 애플리케이션 (egui + eframe)
  - 포장지 이미지(PNG/JPG) 파일 선택
  - 압축 파일(ZIP) 파일 선택
  - 포장하기 버튼 및 출력 경로 저장 다이얼로그
  - 성공/오류 상태 메시지 표시
