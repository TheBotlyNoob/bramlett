release: frontend
    cargo build --release --bin client

frontend:
    pnpm install --prefix crates/client/frontend
    cd crates/client/frontend && pnpm run build

release-win: frontend
    # sudo pacman -S clang
    # cargo install cargo-xwin
    # rustup target add x86_64-pc-windows-msvc
    # rustup component add llvm-tools
    cargo xwin build --release --target x86_64-pc-windows-msvc --bin client

