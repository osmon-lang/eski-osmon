web-install target:
    cd {{ target }} && pnpm install

web-lint target:
    cd {{ target }} && pnpm run lint

rust-install target:
    cd {{ target }} && cargo build

rust-lint target:
    cd {{ target }} && cargo clippy