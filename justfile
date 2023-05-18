# build stuff

build:
    cargo build
    anchor build

make:
    ./scripts/build-all.sh .

tarball:
    ./scripts/ci/create-tarball.sh

clean:
    cargo clean
    rm -rfv bin target lib clockwork-geyser-plugin-release*

re: clean
    just make


# aliases
version:
    cat VERSION

solana-version:
    ./scripts/ci/solana-version.sh

rust-version:
    bash -c 'source ./scripts/ci/rust-version.sh; echo $rust_stable'

kill:
    pkill solana-test-validator

release-patch:
    gh workflow run bump-release.yaml -F bump=patch

cli *args:
    cargo run --bin clockwork {{args}}

localnet *args: build
    cargo run --bin clockwork localnet --dev {{args}}

net:
    cargo run --bin clockwork localnet --dev

logs:
    less test-ledger/validator.log

tlg:
    tail -f test-ledger/validator.log

watch:
    cargo watch -c -x "check"

watch-cli:
    cargo watch -c -x "check --bin clockwork"


# links
pr:
    open https://github.com/clockwork-xyz/clockwork/pulls

actions:
    open https://github.com/clockwork-xyz/clockwork/actions

releases:
    open https://github.com/clockwork-xyz/clockwork/releases

