# Vinting

## About

Vinting is a REST api school project that uses a relational database and has a responsive frontend.

Features:

- [x] Relational database
- [ ] Rest api
- [ ] Responsive frontend (In progress)

## Prerequisites

- [Node.js](https://nodejs.org/en/download) and npm (or npm compatible node package managers like [pnpm](https://pnpm.io/installation) or [bun](https://bun.com/docs/installation))
- [Rust toolchain](https://rust-lang.org/tools/install/)

## Cloning the repo

> [!WARNING]
> This project utilizes git submodules, if you don't clone the submodules you won't be able to compile the app

```sh
git clone https://github.com/SpawnCycle/vinting-server.git --recursive
```

## Running the app

```sh
cargo run
```

Or if you want to run it in release mode

```sh
cargo run --release
```

## Development

- Backend: Nothing special, just rerun/rebuild/retest the app when you're done making changes
- Frontend: Run the backend and then start vite in the `vinting-web` directory with `npm run dev`
