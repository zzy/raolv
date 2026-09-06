# RaoLv - 饶旅

> 饶(ráo), rich, abundant, fulfilled; 旅(lv̌/lǚ), trip, travel, journey.

A media content management site for articles, videos, photos, etc - built on [topcoat](https://github.com/tokio-rs/topcoat) and [surrealdb](https://surrealdb.com/) — all Rust, single binary.

Demo: https://raolv.com

## Stack

- [topcoat](https://github.com/tokio-rs/topcoat) — routing, rendering, sessions, SSE, Tailwind assets
- [surrealdb](https://surrealdb.com/) — schemafull tables, created idempotently at startup
- i18n — locales compiled into the binary; root serves directly, `/{locale}` paths; browser negotiation + cookie memory (en default, zh)

## Features

- Content types: articles, videos, photos, slides
- Media upload with ffmpeg HLS transcoding; orphaned media removed when content is deleted
- Topic tagging, author and admin management
- Sign in / sign up: Argon2id credentials, session + captcha, email activation

## Develop

Requires stable Rust, a surrealdb instance, and topcoat source at `../tmp/topcoat` (pull latest).

```sh
PORT=7700; topcoat dev   # http://127.0.0.1:7700
```

Config lives in `.env` (keys defined in `src/common/config.rs`).

## Deploy

Multi-stage Dockerfile; the topcoat path dependency is swapped to the GitHub main branch at build time.

```sh
docker build -t ghcr.io/zzy/raolv:latest .
docker compose up -d
```

- Port `7700`; uploads volume `/root/raolv-uploads:/app/data/media`
- External network `surrealdb_net`; server `.env` sets `DB_URL` to the surrealdb container
- nginx: no rewrites (locale negotiation is in-app), disable buffering, raise timeouts (SSE, video upload)

## License

[MIT](LICENSE)
