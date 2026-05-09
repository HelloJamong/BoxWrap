# Changelog

## [26.1.1] - 2026-05-09

### Fixed
- 릴리즈 빌드 실행 시 CMD 콘솔 창이 함께 열리는 문제 수정
  - `windows_subsystem = "windows"` 적용 (릴리즈 빌드에만 적용, 디버그 빌드는 콘솔 유지)
- GUI 한글 텍스트가 ㅁ 모양으로 깨지는 문제 수정
  - Windows 기본 한글 폰트 맑은 고딕(`malgun.ttf`) 로드 추가
  - 폰트 로드 실패 시 기본 폰트로 자동 폴백

## [26.1.0] - 2026-05-09

### Added
- ZIP Polyglot 포장 기능
  - PNG/JPG 이미지 안에 ZIP 파일을 삽입하는 핵심 알고리즘 구현
  - EOCD(End of Central Directory) 역방향 탐색 및 Central Directory Entry 오프셋 보정
  - 7-Zip, Windows 탐색기 기본 압축 해제 모두 정상 동작 확인
  - 이미지 뷰어에서는 원본 이미지 그대로 표시 (PNG IEND 이후 데이터 무시 원리 활용)
- 입력 파일 유효성 검사
  - PNG 매직 바이트(`\x89PNG`) 및 JPG 매직 바이트(`\xFF\xD8\xFF`) 검증
  - ZIP 로컬 파일 헤더 시그니처(`PK\x03\x04`) 검증
- Windows GUI 애플리케이션 (egui + eframe)
  - 포장지 이미지(PNG/JPG) 파일 선택 다이얼로그
  - 압축 파일(ZIP) 파일 선택 다이얼로그
  - 두 파일 선택 시에만 활성화되는 포장하기 버튼
  - 출력 파일 저장 경로 선택 다이얼로그
  - 성공/오류 상태 메시지 표시
- GitHub Actions 릴리즈 워크플로우
  - 태그 푸시(`v*.*.*`) 시 `windows-latest`에서 자동 빌드
  - CHANGELOG.md 해당 버전 섹션을 릴리즈 노트로 자동 추출
  - `BoxWrap.exe`를 GitHub Release에 자동 첨부

### Technical
- 단위 테스트 14개 + 통합 테스트 4개 (총 18개)
- 버전 형식: `v{YY}.{major}.{minor}` (예: v26.1.0 = 2026년, 메이저 1, 마이너 0)
