# actix-web-react-template

Actix WebでReactをサーブするテンプレートを作成するコマンド

https://github.com/takumi3488/react-axum-template を元にして作っています

```
cargo install awr-tmpl
awr-tmpl
```

## 特徴

- できるだけバージョン指定しないテンプレートのため、テンプレート自体のメンテナンスが直近で行われていなくても最新版のライブラリを使用できる
- 1つのActix Webアプリで、サーバーサイドAPIとビルド済みのReactフロントエンドをルーティングする仕組みのため、1サーバーでアプリケーション全体が完結する
  - 小規模アプリにおいてフロントエンド用のインフラ管理が必要なく経済的
- 開発環境の2プロセスの立ち上げもTaskfileの1コマンドで可能
- E2Eテスト用のDockerComposeとPlaywrightが導入済み
- 本番ビルド用のDockerfileもあり

## スタック

### サーバーサイド

Actix Webベースでanyhowやtracing等が入っています。

### フロントエンド

FarmのReactテンプレートを用いてプロジェクトを生成します。パッケージマネージャはpnpmを使用します。biomeがセットアップ済みです。
