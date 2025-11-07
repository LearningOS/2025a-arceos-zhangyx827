# Repository Guidelines

## Project Structure & Module Organization
- Workspace root managed by Cargo. Key dirs: `modules/` (core crates), `api/` (public APIs), `ulib/` (user libs like `axstd`, `axlibc`), `examples/`, `exercises/`, `payload/`, `tour/`, `scripts/` (Make/Cargo helpers), `doc/`, `platforms/`.
- Default app path is set by `A` (e.g., `A=examples/helloworld`). Platform/arch tuned via `ARCH`/`PLATFORM`.

## Build, Test, and Development Commands
- `make A=path ARCH=<x86_64|riscv64|aarch64> build` — Build selected app.
- `make run A=... [SMP=4 LOG=info NET=y BLK=y]` — Run under QEMU with options.
- `make debug` / `make gdb` — Start QEMU with GDB; open ELF with `init.gdb`.
- `make unittest` — Run unit tests (`cargo test` for workspace and `axfs` with feature `myfs`).
- `make fmt` / `make clippy` — Format and lint Rust. `make fmt_c` for C in `ulib/axlibc`.
- `make doc` — Build docs. `make disasm` — Disassemble built ELF.

## Coding Style & Naming Conventions
- Rust: follow `rustfmt` defaults; run `cargo fmt --all`. Lint with `clippy` and fix warnings where feasible.
- C (only in `ulib/axlibc`): use repository `.clang-format`; run `make fmt_c`.
- Names: crates and modules `snake_case`; types `UpperCamelCase`; constants `SCREAMING_SNAKE_CASE`.

## Testing Guidelines
- Prefer `cargo test --workspace` locally; CI uses similar. For filesystem tests, ensure `axfs` features align (e.g., `myfs`).
- Name Rust tests with clear behavior intent (e.g., `tests/fs_rename.rs`, `mod tests { ... }`).
- Keep tests deterministic; avoid QEMU runtime in unit tests unless explicitly needed.

## Commit & Pull Request Guidelines
- Commits: imperative mood, concise scope (e.g., `fs: fix rename on ramfs`). Group related changes; avoid mixing refactors with behavior changes.
- PRs: include problem/solution summary, key commands to reproduce (`make ...` / `cargo test`), affected platforms (`ARCH`, `PLATFORM`), and screenshots/log snippets when relevant.
- Link related issues; request reviews for module owners when touching `modules/*` or `api/*`.

## Security & Configuration Tips
- Do not commit build artifacts or images (`*.img`, `*.elf`, `qemu.log`); `.gitignore` already covers most.
- When enabling devices at runtime, prefer flags over code changes: `NET=y`, `BLK=y`, `GRAPHIC=y`.

