set -x
if ! type "wasm-pack" > /dev/null; then
    echo "Could not find wasm-pack"
    exit 1
fi

if ! type "wasm-bindgen" > /dev/null; then
    echo "Could not find wasm-bindgen"
    exit 1
fi

root="buil/web"
wasmOutput="web/pkg"
crateDir="native"
crateName="native"

export RUSTUP_TOOLCHAIN='nightly'
export RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals'
export CARGO_TERM_COLOR='always'

wasm-pack build -t no-modules -d $wasmOutput --no-typescript --out-name $crateName --release $crateDir -- -Z build-std=std,panic_abort
wasm-bindgen $crateDir/target/wasm32-unknown-unknown/release/$crateName.wasm --out-dir $wasmOutput --no-typescript --target no-modules
flutter build web
chmod -c -R +rX "build/web/"