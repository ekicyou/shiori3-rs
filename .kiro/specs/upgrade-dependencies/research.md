# Research & Discovery Log

**Feature**: upgrade-dependencies  
**Date**: 2025-12-25  
**Language**: ja

---

## Summary

依存クレートのメジャー・マイナー版アップグレードと Rust edition 2024 への移行に関する詳細調査。各クレートの互換性、breaking change、リスク要因を特定し、段階的アップグレード戦略の基礎となる情報を収集。

### Key Findings

1. **pest 2.8.4 (マイナー版)**: マイナー版だが構文解析ロジック改善あり。既存 req_parser.pest との互換性検証必須。
2. **thiserror 2.0.17 (メジャー版)**: derive マクロの API が変更される可能性高い。1.0.69（パッチ版）への退却オプション用意。
3. **windows-sys 0.61.2**: 0.52→0.61 は複数マイナー版スキップ。Win32 API シグネチャ確認が重要。
4. **Edition 2024**: 2021 との互換性高い。compiler warning の確認程度で済む見込み。

---

## Research Log

### Topic 1: Rust Edition 2024 への移行

**Investigation**: Rust エディション 2024 の新機能と既存コードへの影響

**References**:
- Rust Language Blog (2024 edition announcement)
- Edition Migration Guide (official Rust docs)

**Findings**:
- Edition 2024 は主に言語機能の追加（async/await 改善、unsafe block 制限等）
- 2021 → 2024 への移行は宣言的（Cargo.toml の edition フィールド変更）
- Breaking change は稀。主に clippy lint による警告検出
- 現在のコード (2021 edition) は 2024 でもコンパイル可能の見込み高い

**Design Impact**:
- Edition 変更は最初のステップとして「最小リスク」扱い
- rustc/clippy 警告の確認・修正が必要な場合あり

---

### Topic 2: anyhow 1.0.75 → 1.0.100 (パッチ版)

**Investigation**: パッチ版アップグレードの互換性確認

**References**:
- anyhow crates.io changelog
- GitHub releases (ekicyou/shiori3-rs での使用パターン)

**Current Usage in Codebase**:
- `src/error.rs`: `use anyhow::Result` による Result wrapper
- `src/api.rs`: `anyhow::Result<T>` の使用（Error type wrapping）

**Findings**:
- anyhow 1.0.x シリーズはパッチ版で breaking change なし
- Result<T> API は安定。互換性リスク最小
- 1.0.75 → 1.0.100 は セキュリティ・パフォーマンス改善のみ

**Design Impact**:
- 低リスク。単純な Cargo.toml 版指定変更で対応可能
- API 変更なし

---

### Topic 3: log 0.4.20 → 0.4.29 (パッチ版)

**Investigation**: ログマクロ API の互換性確認

**References**:
- log crates.io releases
- Macro stability documentation

**Current Usage**:
- `src/api.rs`: `log::info!`, `log::debug!` マクロ使用

**Findings**:
- log 0.4.x はマクロベース API で breaking change なし
- 0.4.20 → 0.4.29 はバグ修正・パフォーマンス改善のみ
- Macro interface 完全互換

**Design Impact**:
- 低リスク。Cargo.toml 変更のみ
- API 互換性なし懸念

---

### Topic 4: pest/pest_derive 2.7.5 → 2.8.4 (マイナー版)

**Investigation**: パーサー生成ロジックの変更影響分析

**References**:
- pest GitHub releases (2.8.0+ changelog)
- pest_derive macro changes
- CHANGELOG.md in pest repository

**Current Usage**:
- `src/parsers/req_parser.pest`: SHIORI リクエスト文法定義
- `src/parsers/req_parser.rs`: pest_derive! マクロから生成されたパーサー
- `src/parsers/req.rs`: パーサー出力の処理

**Key Changes in pest 2.8.4**:
- パーサー最適化 (rule compilation 改善)
- 新しい Pest syntax サポート
- 既存文法との互換性基本的に保持されるが、edge case での挙動変更可能性

**Design Impact**:
- マイナー版だが検証必須
- req_parser.pest を最新文法に適合させる必要の可能性
- テスト（test_data/）による動作確認が重要
- 困難な場合は 2.7.x 最新版への退却オプション

---

### Topic 5: thiserror 1.0.51 → 2.0.17 (メジャー版) または 1.0.69 (パッチ版)

**Investigation**: エラー型定義マクロの API 変更

**References**:
- thiserror GitHub releases (v2.0.0+ CHANGELOG)
- thiserror proc-macro changes
- Migration guide (if available)

**Current Usage**:
- `src/error.rs`: `#[derive(Error)]` マクロで MyError 型定義
- Error trait 実装の自動化

**Breaking Changes in thiserror 2.0**:
- derive マクロの属性構文が若干変更の可能性
- Error Display 実装ロジック変更
- Backtrace サポートの改善

**Design Decision Points**:
1. **Option A (2.0.17 目指し)**: メジャー版で最新機能・改善を享受。derive マクロ互換性確認が必須。
2. **Option B (1.0.69 退却)**: パッチ版で安全に移行。機能は 1.0.x の範囲に限定。

**Design Impact**:
- Design フェーズでの判定が必須
- 実装フェーズで derive マクロの互換性テストを実施
- 困難な場合の 1.0.69 への退却を明記

---

### Topic 6: windows-sys 0.52.0 → 0.61.2 (マイナー版)

**Investigation**: Win32 API バインディングの互換性

**References**:
- windows-sys GitHub releases (0.52 → 0.61 changelog)
- windows-rs / microsoft/windows-rs repository
- Win32 API binding generation process

**Current Usage**:
- `src/hglobal/mod.rs`: HGLOBAL 構造体、メモリ管理 (GlobalAlloc, GlobalFree)
- `src/hglobal/enc.rs`: MultiByteToWideChar, WideCharToMultiByte (文字変換)
- `src/hglobal/windows_api.rs`: Win32 API 定数・関数定義

**Integration Points**:
```
features = [
  "Win32_Foundation",
  "Win32_System_Memory",
  "Win32_Globalization",
]
```

**Changes in windows-sys 0.61.2**:
- 0.52→0.61 は複数マイナー版スキップ（0.53, 0.54, ..., 0.61）
- 各マイナー版で Win32 API binding の改善・追加
- 基本的に backward compatible だが、API signature の細部変更の可能性

**Specific API Concerns**:
1. **GlobalAlloc/GlobalFree**: メモリ確保・解放関数。signature 確認必須
2. **MultiByteToWideChar/WideCharToMultiByte**: CP_UTF8, CP_ACP 定数が変更の可能性
3. **HGLOBAL handle**: タイプ定義の evolution

**Design Impact**:
- 中程度リスク。Win32 API バインディング互換性検証が必須
- 困難な場合は 0.52.x 最新版への退却オプション
- hglobal/ モジュール全体の動作テストが重要

---

### Topic 7: env_logger 0.10.1 → 0.11.8 (マイナー版)

**Investigation**: 開発用ログ初期化の互換性

**References**:
- env_logger crates.io releases
- Logger initialization API stability

**Current Usage**:
- `[dev-dependencies]` (開発時のみ)
- テスト・debug ビルドでの log 出力初期化

**Findings**:
- env_logger 0.10.x → 0.11.x はマイナー版アップグレード
- Logger initialization API (Builder pattern) は stable
- Breaking change は稀

**Design Impact**:
- 低リスク（開発用依存関係）
- Cargo.toml 変更のみで対応可能

---

### Topic 8: 段階的アップグレード戦略の詳細

**Investigation**: 最適な実装順序と検証ポイント

**Design Considerations**:
1. **Edition 2024 → パッチ版 → マイナー版 → メジャー版** の順序が推奨
2. 各ステップで `cargo build` + `cargo test` による検証
3. test_data/ SHIORI サンプルによる integration test

**Parallelization Potential**:
- Edition と各クレートは原則的に独立（段階的実行推奨）
- パッチ版（anyhow, log）は同時に更新可能（検証は個別）
- マイナー版（pest, windows-sys）は個別検証が必須

---

## Design Decisions

### Decision 1: thiserror バージョン戦略

**Question**: メジャー版 2.0.17 か保守版 1.0.69 か？

**Options**:
- **A (推奨)**: 2.0.17 を目指す。derive マクロ互換性確認後に判定。
- **B (退却)**: 互換性問題で 1.0.69 に自動退却。

**Rationale**: Ambitious approach で最新版を目指しつつ、柔軟性を保持。

**Design Impact**: 実装フェーズで derive マクロ互換性テスト実施。

---

### Decision 2: pest 互換性検証方法

**Question**: 2.8.4 への移行確認方法は？

**Approach**:
- req_parser.pest の構文確認
- test_data/ SHIORI サンプルの parse 検証
- 既存テスト suite での動作確認

**Fallback**: 困難な場合は 2.7.x 最新版に退却。

---

### Decision 3: windows-sys Win32 API mapping 検証

**Question**: 0.61.2 への migration で API シグネチャ確認は？

**Approach**:
1. 各関数（GlobalAlloc, MultiByteToWideChar 等）の signature 比較
2. hglobal/ モジュール全体のユニットテスト実施
3. 実際の HGLOBAL allocation/deallocation テスト

**Fallback**: API 互換性問題で 0.52.x 最新版に退却。

---

## Architecture Pattern Evaluation

### Pattern Selection: Staged Crate Upgrade

**Pattern Name**: Dependency Upgrade via Staged Testing

**Rationale**:
- 各クレートのリスク度合いに応じた段階的アップグレード
- 各ステップで完全なテスト (cargo test) による検証
- 問題発生時の即座なロールバック可能性

**Boundary Clarity**:
- Edition upgrade: 言語機能 (Boundary: Cargo.toml)
- Each crate upgrade: 依存関係 version (Boundary: Cargo.toml, lock file)
- Backward compatibility testing: Integration test (Boundary: test_data/)

**Integration Points**:
- 各クレート更新は独立だが、feature flag や cfg gate 確認が必要（Windows-specific）

---

## Risks & Mitigations

| Risk                             | Impact                               | Mitigation                                          |
| -------------------------------- | ------------------------------------ | --------------------------------------------------- |
| pest 2.8.4 macro incompatibility | Parse failure, broken SHIORI parsing | Comprehensive test_data/ validation, 2.7.x fallback |
| thiserror 2.0 derive change      | Compilation error in error.rs        | Derive syntax validation, 1.0.69 fallback           |
| windows-sys API signature change | Unsafe code error, memory issues     | Win32 API signature mapping, 0.52.x fallback        |
| Edition 2024 compiler warning    | Build warnings, potential failures   | Clippy warnings analysis, code adjustment           |
| Transitive dependency conflicts  | Dependency resolution failure        | Cargo.lock deletion, cargo update --aggressive      |

---

## Next Phase: Implementation Planning

### Pre-Implementation Checklist

- [ ] thiserror バージョン判定 (2.0.17 vs 1.0.69)
- [ ] pest 2.8.4 req_parser.pest compatibility mapping
- [ ] windows-sys API signature reference document
- [ ] Edition 2024 compiler/clippy warning list
- [ ] test_data SHIORI サンプル validation plan

### Implementation Task Dependencies

**Sequential (必須)**:
1. Edition 2024 migration
2. anyhow, log, env_logger (パッチ版)
3. pest/pest_derive (マイナー版)
4. thiserror (判定後)
5. windows-sys (マイナー版)
6. Version update to 0.6.6

**Parallel Opportunities**:
- anyhow + log: 同時更新テストは可（検証は個別）
- env_logger: 開発用のため独立テスト

---

## Supporting References

- Rust Edition 2024 Announcement: https://github.com/rust-lang/rust/
- pest changelog: https://github.com/pest-parser/pest/releases
- windows-sys releases: https://github.com/microsoft/windows-rs/
- thiserror v2 migration: https://github.com/dtolnay/thiserror
