# create-svelte

Everything you need to build a Svelte project, powered by [`create-svelte`](https://github.com/sveltejs/kit/tree/main/packages/create-svelte).

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server.
Inside the `src-tauri` folder:

```bash
cargo tauri dev
```

## Setup

After cloning the repo you need to follow the usual steps like `npm install` (or `pnpm install` or `yarn`) and install the `tauri-cli`.
Inside the `src-tauri` folder:

```bash
cargo install tauri-cli
```

## SurrealDB

You need an instance of SurrealDB running, you can use docker:

```bash
docker run --rm --pull always -p 8000:8000 surrealdb/surrealdb:latest start
```

If you want to persist your data follow the official documentation: https://surrealdb.com/docs/surrealdb/installation/running/docker
