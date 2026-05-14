dev-clippy:
    bacon clippy -- --all-targets --all-features -- -D warnings

dev-app:
    cd assember && pnpm tauri dev
build-app:
    cd assembler && pnpm tauri build
