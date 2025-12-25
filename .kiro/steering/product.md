# Product Overview

SHIORI/3.0 仕様を Rust で実装した共有ライブラリ。ゴーストアプリケーション（ ukagaka）とベースウェア間の通信・動作仕様を規定し、キャラクターの対話エンジンとして機能する。

## Core Capabilities

- **SHIORI/3.0 プロトコル対応**: ベースウェアとの標準的な通信インターフェースを実装
- **リクエスト解析**: HTTP ライクな SHIORI プロトコルのリクエストをパースし、応答を返す
- **マルチバイト文字対応**: Windows HGLOBAL を通じた文字列管理とエンコーディング変換
- **メモリ管理**: Windows HGLOBAL 経由での安全な動的メモリ確保・解放
- **型安全な Rust 実装**: Rust の所有権システムにより安全で効率的なコード

## Target Use Cases

- ゴーストアプリケーション開発者が、Rust で SHIORI インターフェースを実装する
- ベースウェアが Rust で実装されたゴーストとの通信を行う
- SHIORI/3.0 仕様準拠の新規ゴースト作成

## Value Proposition

Windows 環境での HGLOBAL メモリ管理と文字エンコーディングを Rust の型システムで安全に扱い、従来の C/C++ 実装に比べてメモリ安全性と開発効率を向上させる。

---
_Focus on patterns and purpose, not exhaustive feature lists_
