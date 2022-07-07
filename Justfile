# Builds `idb` and `idb-sys`
build:
    @echo 'Building...'
    cargo build

alias test := test-chrome

# Runs browser tests for `idb` using chrome
test-chrome:
    @echo 'Testing...'
    cd idb && wasm-pack test --chrome

# Runs browser tests for `idb` using chrome (intended for use in CI)
test-chrome-headless:
    @echo 'Testing...'
    cd idb && wasm-pack test --headless --chrome

# Runs browser tests for `idb` using firefox (intended for use in CI)
test-firefox-headless:
    @echo 'Testing...'
    cd idb && wasm-pack test --headless --firefox

# Generate readme from doc comments
readme:
    @echo 'Generating README...'
    cd idb && cargo readme > ../README.md
