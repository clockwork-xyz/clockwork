# build stuff

make:
    ./scripts/build-all.sh .

tarball:
    ./scripts/create-tarball.sh

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

cli:
    cargo run --bin clockwork

pr:
    open https://github.com/clockwork-xyz/clockwork/pulls

actions:
    open https://github.com/clockwork-xyz/clockwork/actions

releases:
    open https://github.com/clockwork-xyz/clockwork/releases
