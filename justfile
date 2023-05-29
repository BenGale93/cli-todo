deploy:
    @cargo build --release
    @cp target/release/todo ~/.cargo/bin

coverage:
    @cargo tarpaulin -v --follow-exec --skip-clean

test:
    @cargo nextest run
