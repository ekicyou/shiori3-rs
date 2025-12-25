# Project Structure

## Organization Philosophy

モジュール駆動。SHIORI 通信インターフェース（api）、プロトコルパーサー（parsers）、メモリ管理（hglobal）、エラーハンドリング（error）を独立したモジュールとして設計。各モジュールは明確な責任を持ち、依存関係を最小化。

## Directory Patterns

### `/src`
**Purpose**: ライブラリの主要ロジック  
**Organization**:
- `lib.rs` - モジュール宣言と pub use による公開 API 定義
- `api.rs` - Shiori3 トレイト定義、RawShiori3 / Shiori3Impl の実装
- `error.rs` - MyError 型定義と エラーハンドリング
- `parsers/` - SHIORI プロトコルパーサー（pest 文法）
- `hglobal/` - Windows HGLOBAL ラッパーとエンコーディング変換

### `/src/parsers`
- `req_parser.pest` - SHIORI リクエスト文法定義（pest）
- `req_parser.rs` - パーサー実装
- `req.rs` - ShioriRequest 構造体と解析ロジック
- `test_data/` - SHIORI 2.1/3.0/3.1/3.2 サンプルリクエスト

### `/src/hglobal`
- `mod.rs` - ShioriString 構造体（HGLOBAL ラッパー）
- `enc.rs` - Encoding/Encoder 定義
- `windows_api.rs` - Windows API ラッパー（マルチバイト変換）

### `/tests`
**Purpose**: Integration テスト  
**Example**: パーサーの完全フロー、API 呼び出し検証

## Naming Conventions

- **Modules**: snake_case (`api`, `error`, `hglobal`, `parsers`)
- **Types**: PascalCase (`Shiori3`, `ShioriRequest`, `MyError`)
- **Functions/Methods**: snake_case (`load`, `request`, `parse`)
- **Constants/Enums**: UPPER_SNAKE_CASE または PascalCase

## Import Organization

```rust
// モジュール宣言
mod api;
mod error;
mod hglobal;
mod parsers;

// 公開 API 再エクスポート
pub use crate::api::{RawShiori3, Shiori3};
pub use crate::error::{MyError as ShioriError, MyResult as ShioriResult};
pub use crate::hglobal::{ShioriString, Encoder, Encoding};
pub use crate::parsers::req::ShioriRequest;
```

**Module Visibility**:
- pub: API のみ公開（実装詳細は非公開）
- pub(crate): ライブラリ内での共有

## Code Organization Principles

- **モジュール独立**: 各モジュールは最小限の依存
- **責任分離**: api（インターフェース）、parsers（解析）、hglobal（メモリ）は独立
- **エラーハンドリング**: 全関数は Result 返却
- **型安全**: 不安全操作は unsafe ブロックに集約
- **テスト駆動**: 単体テスト + integration テスト

---
_Document patterns, not file trees. New files following patterns shouldn't require updates_
