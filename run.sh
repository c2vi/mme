
set -e


wasm-pack build --dev --features wasm-target --no-default-features


cd /home/me/work/mize

wasm-pack build --target bundler --dev --features wasm-target --no-default-features #--out-dir ~/work/mize/src/platform/wasm/npm_pkg/generated
#export CARGO_BUILD_RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals'
#wasm-pack build --target no-modules --dev . -Z build-std=panic_abort,std --features wasm-target --no-default-features --config 'build.rustflags = "-C target-feature=+atomics,+bulk-memory,+mutable-globals"'
cp -r ~/work/mize/pkg/* ~/work/mize/src/platform/wasm/npm_pkg/generated

#RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' cargo build --target wasm32-unknown-unknown -Z build-std=panic_abort,std --lib --no-default-features
#cp ~/work/mize/target/wasm32-unknown-unknown/debug/mize.wasm ~/work/mize/src/platform/wasm/npm_pkg/generated/mize_bg.wasm

cd /home/me/work/mme-presenters/presenters/mme-js/
npm run build -- --mode development


cargo build --manifest-path ~/work/modules/modules/mme/Cargo.toml
cargo build --manifest-path ~/work/modules/modules/String/Cargo.toml


cargo run --manifest-path ~/work/mize/Cargo.toml -- gui
