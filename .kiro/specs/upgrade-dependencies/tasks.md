# Implementation Plan

## Task Breakdown

### Phase 1: Edition & Patch Upgrades

- [x] 1. Rust Edition 2024 移行
- [x] 1.1 Edition 2024 への変更と検証
  - Cargo.toml の edition フィールドを "2024" に変更
  - `cargo build` でコンパイル成功確認
  - `cargo clippy --all` で新規警告を確認・対応
  - `cargo test` で既存テスト suite が成功することを確認
  - 変更をコミット
  - _Requirements: 1_

- [x] 2. パッチバージョン依存関係のアップグレード
- [x] 2.1 (P) anyhow を 1.0.100 にアップグレード
  - Cargo.toml の anyhow バージョンを "1.0.100" に変更
  - `cargo build` でコンパイル成功確認
  - src/error.rs, src/api.rs の Result 型互換性確認
  - `cargo test` で既存テストが成功することを確認
  - 変更をコミット
  - _Requirements: 2_

- [x] 2.2 (P) log を 0.4.29 にアップグレード
  - Cargo.toml の log バージョンを "0.4.29" に変更
  - `cargo build` でコンパイル成功確認
  - src/api.rs のログマクロ互換性確認
  - `cargo test` で既存テストが成功することを確認
  - 変更をコミット
  - _Requirements: 3_

- [x] 2.3 (P) env_logger を 0.11.8 にアップグレード
  - Cargo.toml の [dev-dependencies] env_logger バージョンを "0.11.8" に変更
  - `cargo build` でコンパイル成功確認
  - Logger 初期化の互換性確認
  - `cargo test` で既存テストが成功することを確認
  - 変更をコミット
  - _Requirements: 7_

### Phase 2: Minor Version Upgrades

- [x] 3. pest/pest_derive を 2.8.4 にアップグレード
- [x] 3.1 pest/pest_derive バージョン更新と検証
  - Cargo.toml の pest, pest_derive バージョンを "2.8.4" に変更
  - `cargo build` で req_parser.rs 生成コード確認
  - src/parsers/req_parser.pest 構文の互換性確認
  - test_data/ の SHIORI 2.1/3.0/3.1/3.2 サンプルでパーステスト実行
  - `cargo test` で既存テストが成功することを確認
  - 成功時: 変更をコミット
  - 失敗時: Fallback タスク 3.2 へ
  - _Requirements: 4_

- [ ] 3.2 (Fallback) pest/pest_derive を 2.7.x 最新版にアップグレード
  - 2.8.4 で互換性問題が解決困難な場合のみ実行
  - Cargo.toml の pest, pest_derive バージョンを 2.7.x の最新版に変更
  - `cargo build` でコンパイル成功確認
  - `cargo test` で既存テストが成功することを確認
  - 変更をコミット
  - _Requirements: 4_

- [x] 4. windows-sys を 0.61.2 にアップグレード
- [x] 4.1 windows-sys API 互換性調査
  - 0.52.0 と 0.61.2 の windows-sys docs で API signature を比較
  - GlobalAlloc, GlobalFree, MultiByteToWideChar, WideCharToMultiByte の型変更確認
  - src/hglobal/windows_api.rs の unsafe block で影響範囲を特定
  - 必要な修正内容を記録
  - _Requirements: 6_

- [x] 4.2 windows-sys バージョン更新と修正
  - Cargo.toml の windows-sys バージョンを "0.61.2" に変更
  - タスク 4.1 で特定した修正を src/hglobal/windows_api.rs に適用
  - `cargo build` でコンパイル成功確認
  - src/hglobal/ の unit test で HGLOBAL allocation/deallocation, encoding 確認
  - `cargo test` で既存テストが成功することを確認
  - 成功時: 変更をコミット
  - 失敗時: Fallback タスク 4.3 へ
  - _Requirements: 6_

- [ ] 4.3 (Fallback) windows-sys を 0.52.x 最新版にアップグレード
  - 0.61.2 で互換性問題が解決困難な場合のみ実行
  - Cargo.toml の windows-sys バージョンを 0.52.x の最新版に変更
  - `cargo build` でコンパイル成功確認
  - `cargo test` で既存テストが成功することを確認
  - 変更をコミット
  - _Requirements: 6_

### Phase 3: Staged thiserror Upgrade

- [x] 5. thiserror Stage 1: マイナー版最新 (1.0.69) へアップグレード
- [x] 5.1 thiserror 1.0.69 へのアップグレード
  - Cargo.toml の thiserror バージョンを "1.0.69" に変更
  - `cargo build` でコンパイル成功確認
  - src/error.rs の #[derive(Error)] マクロ互換性確認
  - Error type の Display/Debug 実装確認
  - `cargo test` で error handling テストが成功することを確認
  - 変更をコミット
  - _Requirements: 5_

- [x] 6. thiserror Stage 2: メジャー版 (2.0.17) へのアップグレード (Optional)
- [x] 6.1 thiserror 2.0.17 互換性検証
  - #[error] attribute syntax の変更有無を確認（macro 展開結果の比較）
  - #[source] / #[from] attribute の互換性を確認
  - Display/Debug trait 自動実装の動作を確認
  - Breaking change が src/error.rs の修正で対応可能か判断
  - 判断結果を記録（互換性問題の有無と修正難易度）
  - _Requirements: 5_

- [x] 6.2 thiserror 2.0.17 への更新または 1.0.69 維持の決定
  - タスク 6.1 の判断結果に基づき実行パスを選択
  - **2.0.17 採択パス**: すべての互換性チェックが ✅ の場合
    - Cargo.toml の thiserror バージョンを "2.0.17" に変更
    - 必要に応じて src/error.rs を修正
    - `cargo build` でコンパイル成功確認
    - `cargo test` で error handling テストが成功することを確認
    - 変更をコミット: "Upgrade thiserror to 2.0.17 (major)"
  - **1.0.69 維持パス**: 修正困難な互換性問題がある場合
    - Cargo.toml の thiserror は "1.0.69" のまま維持
    - 判断理由をコミットメッセージに記録: "Keep thiserror at 1.0.69 (stable)"
  - _Requirements: 5_

### Phase 4: Version Update & Final Validation

- [x] 7. プロジェクトバージョンを 0.6.6 に更新
- [x] 7.1 バージョン更新と最終検証
  - Cargo.toml の version フィールドを "0.6.6" に変更
  - `cargo build --release` でリリースビルド成功確認
  - `cargo test` で既存テスト suite 全体が成功することを確認
  - バージョン情報がバイナリに反映されていることを確認
  - 変更をコミット
  - _Requirements: 8_

- [x] 7.2* (Optional) 統合テストと受入基準の追加検証
  - Requirement 1 の Acceptance Criteria (edition 2024 設定) 再確認
  - Requirement 2-7 の Acceptance Criteria (各依存関係アップグレード) 再確認
  - Requirement 8 の Acceptance Criteria (バージョン 0.6.6 反映) 再確認
  - test_data/ SHIORI samples の完全パーステスト実行
  - すべての要件が満たされていることを最終確認
  - _Requirements: 1, 2, 3, 4, 5, 6, 7, 8_

## Requirements Coverage

- **Requirement 1** (Edition 2024): Task 1.1
- **Requirement 2** (anyhow): Task 2.1
- **Requirement 3** (log): Task 2.2
- **Requirement 4** (pest/pest_derive): Task 3.1, 3.2
- **Requirement 5** (thiserror): Task 5.1, 6.1, 6.2
- **Requirement 6** (windows-sys): Task 4.1, 4.2, 4.3
- **Requirement 7** (env_logger): Task 2.3
- **Requirement 8** (Version 0.6.6): Task 7.1

## Task Dependencies

**Sequential Execution Required**:
1. Task 1 (Edition) → Task 2 (Patch) → Task 3, 4 (Minor) → Task 5, 6 (thiserror Staged) → Task 7 (Version)
2. Task 4.1 (API 調査) → Task 4.2 (更新) または Task 4.3 (Fallback)
3. Task 5.1 (Stage 1) → Task 6.1 (互換性検証) → Task 6.2 (Stage 2 決定)

**Parallel Execution Possible**:
- Task 2.1, 2.2, 2.3 (パッチバージョンアップグレード) は並列実行可能
- Task 3, 4 (pest, windows-sys) は Task 2 完了後に並列実行可能

**Fallback Paths**:
- Task 3.1 失敗 → Task 3.2 (pest 2.7.x fallback)
- Task 4.2 失敗 → Task 4.3 (windows-sys 0.52.x fallback)
- Task 6.2: 互換性判断により 2.0.17 または 1.0.69 維持を選択
