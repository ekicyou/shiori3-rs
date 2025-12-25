# Requirements Document

## Project Description (Input)

SHIORI/3.0 Rust 実装ライブラリの依存クレートをすべて最新版にアップグレードし、Rust edition を 2024 に移行する。各アップグレードは小さなループで段階的に実施し、コンパイル・テスト・コミットで検証する。最終的にプロジェクトバージョンを 0.6.6 に更新する。

## Requirements

### Requirement 1: Rust Edition をバージョン 2024 にアップグレード

**Objective**: メンテナンス担当者として、最新の Rust エディション機能を利用できるようにしたい。そうすることで、言語の最新の改善と最適化を活用できる。

#### Acceptance Criteria

1. When Cargo.toml の edition 値を "2024" に変更したとき、The プロジェクトは `cargo build` でコンパイルエラーなく成功すること
2. When Cargo.toml の edition が 2024 に設定されたとき、The 既存テストスイートはすべて `cargo test` で成功すること
3. When edition 変更が完了したとき、The コード変更がコミット記録に残ること

### Requirement 2: 依存クレート anyhow を最新版にアップグレード

**Objective**: メンテナンス担当者として、エラーハンドリングライブラリの最新機能を利用できるようにしたい。そうすることで、セキュリティと安定性の向上を享受できる。

#### Acceptance Criteria

1. When Cargo.toml の anyhow バージョンを最新版に変更したとき、The プロジェクトは `cargo build` でコンパイルエラーなく成功すること
2. When anyhow のアップグレードが完了したとき、The 既存テストスイートはすべて `cargo test` で成功すること
3. If anyhow のアップグレード後に API 互換性がない場合、The コード側で対応を実施し、テストが成功すること

### Requirement 3: 依存クレート log を最新版にアップグレード

**Objective**: メンテナンア担当者として、ログング機能の最新版を利用できるようにしたい。そうすることで、ログング性能とセキュリティを向上できる。

#### Acceptance Criteria

1. When Cargo.toml の log バージョンを最新版に変更したとき、The プロジェクトは `cargo build` でコンパイルエラーなく成功すること
2. When log のアップグレードが完了したとき、The 既存テストスイートはすべて `cargo test` で成功すること

### Requirement 4: 依存クレート pest/pest_derive を最新版(2.8.4)にアップグレード

**Objective**: メンテナア担当者として、パーサージェネレータの最新版を利用できるようにしたい。そうすることで、パーサーの機能と安定性を向上できる。

**Note**: 最新版 2.8.4 (マイナー版更新) を目指す。実装フェーズで互換性問題が解決困難な場合は、2.7.x の最新版に留める。

#### Acceptance Criteria

1. When Cargo.toml の pest バージョンを 2.8.4 に変更したとき、The プロジェクトは `cargo build` でコンパイルエラーなく成功すること
2. When Cargo.toml の pest_derive バージョンを 2.8.4 に変更したとき、The プロジェクトは `cargo build` でコンパイルエラーなく成功すること
3. When pest/pest_derive のアップグレードが完了したとき、The 既存テストスイートはすべて `cargo test` で成功すること
4. If 2.8.4 への移行が困難な場合、The 2.7.x の最新版での動作確認で要件を満たすこと

### Requirement 5: 依存クレート thiserror を最新版(2.0.17)にアップグレード

**Objective**: メンテナア担当者として、エラー型定義ライブラリの最新版を利用できるようにしたい。そうすることで、エラーハンドリングの効率性を向上できる。

**Note**: 最新版 2.0.17 (メジャー版) を目指す。derive マクロの互換性問題が解決困難な場合は、1.0.69 (パッチ版最新) に留める。

#### Acceptance Criteria

1. When Cargo.toml の thiserror バージョンを 2.0.17 に変更したとき、The プロジェクトは `cargo build` でコンパイルエラーなく成功すること
2. When thiserror のアップグレードが完了したとき、The 既存テストスイートはすべて `cargo test` で成功すること
3. If 2.0.17 への移行が困難な場合、The 1.0.69 (パッチ版最新) での動作確認で要件を満たすこと

### Requirement 6: 依存クレート windows-sys を最新版(0.61.2)にアップグレード

**Objective**: メンテナア担当者として、Windows API バインディングの最新版を利用できるようにしたい。そうすることで、Windows サポートの安定性と機能を向上できる。

**Note**: 最新版 0.61.2 (マイナー版更新) を目指す。Win32 API 互換性問題が解決困難な場合は、0.52.x の最新版に留める。

#### Acceptance Criteria

1. When Cargo.toml の windows-sys バージョンを 0.61.2 に変更したとき、The プロジェクトは `cargo build` でコンパイルエラーなく成功すること
2. When windows-sys のアップグレードが完了したとき、The 既存テストスイートはすべて `cargo test` で成功すること
3. If windows-sys のアップグレード後に API 互換性がない場合、The コード側で対応を実施し、テストが成功すること
4. If 0.61.2 への移行が困難な場合、The 0.52.x の最新版での動作確認で要件を満たすこと

### Requirement 7: 開発依存クレート env_logger を最新版にアップグレード

**Objective**: メンテナンア担当者として、開発時ログ出力機能の最新版を利用できるようにしたい。そうすることで、開発効率とデバッグ性を向上できる。

#### Acceptance Criteria

1. When Cargo.toml の env_logger バージョンを最新版に変更したとき、The プロジェクトは `cargo build` でコンパイルエラーなく成功すること
2. When env_logger のアップグレードが完了したとき、The 既存テストスイートはすべて `cargo test` で成功すること

### Requirement 8: プロジェクトバージョンを 0.6.6 にアップグレード

**Objective**: メンテナンア担当者として、すべての依存関係の更新を反映したバージョン情報を提供したい。そうすることで、ユーザーが新しいリリースとその変更内容を把握できる。

#### Acceptance Criteria

1. When すべてのアップグレードが完了したとき、The Cargo.toml の version フィールドが "0.6.6" に更新されること
2. When バージョン更新が完了したとき、The `cargo build` はコンパイルエラーなく成功すること
3. When バージョン更新が完了したとき、The 既存テストスイートはすべて `cargo test` で成功すること
