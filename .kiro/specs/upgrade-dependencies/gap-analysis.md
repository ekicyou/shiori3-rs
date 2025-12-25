# Implementation Gap Analysis

## Feature: upgrade-dependencies

**Analysis Date**: 2025-12-25  
**Feature**: Rust edition 2024 移行と依存クレート全体の最新版アップグレード

---

## 1. Current State Investigation

### 既存コードベース構造

**モジュール構成**:
- `src/api.rs` - SHIORI3 トレイト、RawShiori3 実装
- `src/error.rs` - エラーハンドリング（thiserror 使用）
- `src/hglobal/` - Windows HGLOBAL ラッパー（windows-sys 使用）
- `src/parsers/` - SHIORI リクエストパーサー（pest/pest_derive 使用）

**テスト構成**:
- `src/parsers/test_data/` - SHIORI 2.1/3.0/3.1/3.2 サンプルリクエスト
- `cargo test` で実行可能

**使用されている依存クレート**（調査済み）:
| Crate       | 現在版 | 最新版    | 更新幅       | 用途                                      |
| ----------- | ------ | --------- | ------------ | ----------------------------------------- |
| anyhow      | 1.0.75 | 1.0.100   | パッチ       | 汎用エラーハンドリング（Result に wrap）  |
| log         | 0.4.20 | 0.4.29    | パッチ       | ログ出力（api.rs で log::* マクロ使用）   |
| pest        | 2.7.5  | **2.8.4** | **マイナー** | パーサージェネレータ（req_parser.rs）     |
| pest_derive | 2.7.5  | **2.8.4** | **マイナー** | pest 用マクロ導出                         |
| thiserror   | 1.0.51 | 2.0.17†   | **メジャー** | MyError 型定義（エラー型自動実装）        |
| windows-sys | 0.52.0 | 0.61.2    | マイナー     | Win32 API バインディング（HGLOBAL, 変換） |
| env_logger  | 0.10.1 | 0.11.8    | マイナー     | 開発時ログ設定（dev-dependencies）        |

†thiserror は 1.0.69 (パッチ) か 2.0.17 (メジャー) を選択可能

### 現在の Rust Edition

- **Edition**: 2021（2024 未対応）
- **実装への影響**: 小さい（edition 変更は言語機能のサポートのみ）

---

## 2. Requirements Feasibility Analysis

### 要件対応分析（最新バージョン確認済み）

**Requirement 1-8**:
- Edition 2024 への移行 → **実現可能**（Cargo.toml 1 行編集）
- anyhow (1.0.75 → 1.0.100) → **実現可能**（パッチ版、API 安定）
- log (0.4.20 → 0.4.29) → **実現可能**（パッチ版、マクロ API 安定）
- pest/pest_derive (2.7.5 → **2.8.4**) → **要検証**（**マイナー版更新**、パーサー生成ロジック変更の可能性）
- thiserror (1.0.51 → **2.0.17** または 1.0.69) → **要判断**（**メジャー版選択肢あり**、derive マクロ互換性確認）
- windows-sys (0.52.0 → 0.61.2) → **要検証**（マイナー版、Win32 API シグネチャ確認）
- env_logger (0.10.1 → 0.11.8) → **実現可能**（マイナー版、開発時のみ）
- バージョン 0.6.6 への更新 → **自動的**（Cargo.toml 編集のみ）

### 重要な更新幅の判定

| クレート    | 現在→最新         | 分類           | リスク |
| ----------- | ----------------- | -------------- | ------ |
| anyhow      | 1.0.75→1.0.100    | パッチ版       | 🟢 低   |
| log         | 0.4.20→0.4.29     | パッチ版       | 🟢 低   |
| pest        | 2.7.5→**2.8.4**   | **マイナー版** | 🟡 中   |
| pest_derive | 2.7.5→**2.8.4**   | **マイナー版** | 🟡 中   |
| thiserror   | 1.0.51→**2.0.17** | **メジャー版** | 🔴 高   |
| windows-sys | 0.52.0→0.61.2     | マイナー版     | 🟡 中   |
| env_logger  | 0.10.1→0.11.8     | マイナー版     | 🟢 低   |

### 技術的制約

1. **thiserror メジャー版選択肢**:
   - 1.0.69（安全なパッチアップ）vs 2.0.17（メジャーアップ）
   - 2.0 では derive マクロが変更の可能性
   - Design フェーズで判定必要

2. **Windows プラットフォーム依存**:
   - windows-sys は Windows-only バインディング
   - cfg(windows) ガード下で使用
   - 0.52→0.61 は 9 段階の マイナー更新（注意深い検証必要）

3. **pest パーサー（マイナー版更新）**:
   - req_parser.pest で文法を定義
   - 生成されたパーサーコードは自動化
   - メジャーバージョン更新で構文変更の可能性

3. **テスト駆動アップグレード**:
   - 既存テスト（test_data/ サンプル）で各ステップ検証
   - テスト成功がアップグレード確認条件

---

## 3. Implementation Approach Analysis

### Option A: 段階的アップグレード（推奨）

**戦略**: 各クレートを個別にアップグレードし、コンパイル・テスト・コミットで検証

**推奨実行順序**:
1. **Edition 2024 へ移行** → リスク最小（言語機能変更のみ）
2. **anyhow (1.0.75→1.0.100)** → パッチ版、リスク低い
3. **log (0.4.20→0.4.29)** → パッチ版、リスク低い
4. **env_logger (0.10.1→0.11.8)** → マイナー版だが開発時のみ
5. **thiserror (1.0.51→1.0.69 or 2.0.17)** → **判定必要**：メジャー版か保守版か
6. **pest/pest_derive (2.7.5→2.8.4)** → **マイナー版、検証必須**（パーサー生成コード確認）
7. **windows-sys (0.52.0→0.61.2)** → **マイナー版、リスク中**（Win32 API 互換性確認が最重要）
8. **バージョン更新** → 0.6.6 に設定

**各ステップの検証**:
```bash
# ステップごと
1. Cargo.toml の単一クレートを更新
2. cargo build → コンパイル成功確認
3. cargo test → テスト成功確認
4. git commit -m "Upgrade [crate-name] to latest"
```

**メリット**:
- ✅ 各ステップで問題を隔離できる
- ✅ コミット履歴が細粒度（ロールバック容易）
- ✅ CI/テストで各段階を独立検証
- ✅ 変更の影響範囲が小さい

**デメリット**:
- ❌ 複数回の build/test 実行（時間増加）

### Option B: 一括アップグレード

**戦略**: すべての Cargo.toml 更新を一度に行い、単一の build/test で検証

**メリット**:
- ✅ 実行時間が短い
- ✅ 全体的なバージョン整合性が明確

**デメリット**:
- ❌ 複数クレート問題が同時に発生する可能性
- ❌ どのクレートが問題の原因か特定困難
- ❌ ロールバック時の対応が複雑

**結論**: Option A（段階的）が推奨

### Option C: スキップ（非推奨）

現在バージョンの使用継続。セキュリティリスク増加のため非推奨。

---

## 4. Integration Points & Risks

### 重要な統合ポイント

1. **windows-sys インテグレーション**:
   - `hglobal/mod.rs`, `hglobal/windows_api.rs` で多用
   - HGLOBAL, MultiByteToWideChar, WideCharToMultiByte 等の Win32 API
   - バージョン 0.52 → 最新版への移行で API シグネチャ確認必須

2. **pest パーサー**:
   - `parsers/req_parser.pest` で SHIORI リクエスト文法定義
   - `parsers/req_parser.rs` 生成コード
   - 新バージョンで文法構文変更の可能性

3. **エラーハンドリング**:
   - anyhow/thiserror は Result ベース（API 安定性高い）
   - 既存コード変更最小限で済む可能性高い

### リスク評価

| リスク                        | 影響 | 対策                                             |
| ----------------------------- | ---- | ------------------------------------------------ |
| **windows-sys API 非互換**    | 高   | 各関数シグネチャを確認、必要に応じてラッパー修正 |
| **pest 文法変更**             | 中   | req_parser.pest を最新文法に適合させる           |
| **Edition 2024 未対応コード** | 低   | コンパイルエラーで検出可能                       |
| **テスト失敗**                | 中   | 既存テストで検出、SHIORI リクエスト仕様確認      |

---

## 5. Effort & Complexity Estimation

### 実装規模

- **Edition 2024 移行**: 1-3 時間（Cargo.toml 1 行編集 + 検証）
- **anyhow/log/thiserror/env_logger**: 1-2 時間（API 互換性高い）
- **pest/pest_derive**: 2-4 時間（文法確認・修正が発生の可能性）
- **windows-sys**: 2-6 時間（Win32 API 互換性確認・修正）
- **バージョン更新**: 0.5 時間（Cargo.toml 編集）

**総予定時間**: 6-15 時間（段階的実行、問題なければ 6-8 時間）

### 複雑度

- **総体的**: **中程度**
- **Edition 2024**: 低リスク
- **pest/windows-sys**: 中~高リスク（API 互換性確認必須）

---

## 6. Research Needed for Design Phase

以下の項目は design フェーズで詳細調査：

1. **pest 2.8.4 の API 変更**: マイナー版更新のため構文互換性確認、req_parser.rs の動作確認
2. **windows-sys 0.61.2 の API**: 0.52→0.61 は複数マイナー版スキップのため、Win32 API シグネチャを個別確認（特に MultiByteToWideChar, WideCharToMultiByte）
3. **thiserror メジャー版判定**: 1.0.69（保守版パッチ）vs 2.0.17（メジャー版）の選択。derive マクロ互換性を確認
4. **Edition 2024 での警告**: rustc/clippy の新規警告を確認・修正
5. **依存関係の連鎖**: 各クレートが引き込む他クレートのバージョン確認（特に pest の汎用依存）

---

## 7. Recommendations

### 推奨実装戦略（Ambitious Approach）

1. **段階的アップグレード** Option A を採用（ユーザー指定）
2. **推奨実行順序**（リスク度に基づく）:
   - **最優先（低リスク）**: Edition 2024 → anyhow (1.0.100) → log (0.4.29) → env_logger (0.11.8)
   - **次点（中~高リスク、要検証）**: pest/pest_derive (2.8.4目指し、困難なら 2.7.x) → windows-sys (0.61.2 目指し、困難なら 0.52.x) → thiserror (2.0.17 目指し、困難なら 1.0.69)
   - **最後（自動）**: バージョン 0.6.6 更新

3. **各ステップで実施**:
   - `cargo build` でコンパイル確認
   - `cargo test` でテスト実行
   - `git commit` で記録

4. **段階的アップグレードの柔軟性（困難時の退却戦略）**:
   - **pest/pest_derive**: 2.8.4 → 2.7.x の最新へ退却可能
   - **thiserror**: 2.0.17 (メジャー) → 1.0.69 (パッチ) へ退却可能
   - **windows-sys**: 0.61.2 → 0.52.x の最新へ退却可能
   - 各退却時はテスト成功で要件満たす（Requirements 参照）

5. **予期しない問題時**:
   - Cargo.lock を削除して `cargo update` で依存関係を再解決
   - GitHub Issue / Crate 公開情報を参照
   - 問題クレートは Recommendations 注記の「退却基準」に従う

---

## 8. Implementation Readiness

**Gap Analysis 結論**:

✅ **Ambitious Target**: すべての最新版への移行を目指す  
✅ **段階的アップグレード戦略で低リスク実装**  
✅ **柔軟な退却戦略**: 困難時はマイナー版最新に自動退却可能  
⚠️ **中リスククレート**: pest (2.8.4), windows-sys (0.61.2), thiserror (2.0.17) - 互換性検証必須  
📋 **退却基準**: Requirements ドキュメント参照（各要件に「困難時の退却条件」記載）  

**Design フェーズへの移行**: 推奨可能
