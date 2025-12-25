# Technology Stack

## Architecture

モジュール分割アーキテクチャ。API レイヤー、パーサー、Windows HGLOBAL ラッパー、エラーハンドリングを独立モジュールとして整理し、機能別に責任を分離。

## Core Technologies

- **Language**: Rust 2021 edition
- **Package Manager**: Cargo
- **Platform**: Windows (Win32 API via windows-sys)
- **Minimum Rust Version**: 1.56+（Cargo.lock により管理）

## Key Libraries

- **pest**: パーサージェネレータ（SHIORI リクエスト構文解析）
- **windows-sys**: Windows API バインディング（HGLOBAL, マルチバイト変換）
- **thiserror**: エラー型定義と自動実装
- **log**: ログ出力（本体）
- **env_logger**: ログ設定（開発用）

## Development Standards

### Type Safety
- Unsafe ブロックは Windows API 呼び出しのみに限定
- Result/Option の明示的なハンドリング
- 強い型による API 契約の表現

### Code Quality
- モジュール単位での分離（api, error, hglobal, parsers）
- 公開 API は明示的に pub use で再エクスポート
- テストは integration 形式（/tests ディレクトリ）

### Testing
- SHIORI リクエストパーサーテスト（test_data/ に SHIORI2/3 サンプル保有）
- Windows API 変換ロジックのユニットテスト
- 本体 `cargo test` にて実行

## Development Environment

### Required Tools
- Rust toolchain (rustc, cargo)
- Windows SDK (for Win32 API headers)

### Common Commands
```bash
# Build: cargo build
# Run tests: cargo test
# Generate docs: cargo doc --open
```

## Key Technical Decisions

- **pest を選択**: 文法駆動の宣言的パーサー定義（req_parser.pest）
- **windows-sys**: 安全で最小限な Win32 API バインディング
- **HGLOBAL ラッパー**: Rust の Deref/Drop で RAII パターン実装
- **エラー型**: thiserror で型安全なエラーハンドリング

---
_Document standards and patterns, not every dependency_
