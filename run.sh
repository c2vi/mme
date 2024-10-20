
set -e

# override, where mize loads the mme module from
export MIZE_MODULE_PATH=/home/me/work/mme:/home/me/work/modules:/home/me/work/presenters
export MIZE_MODULE_NO_REPO=1
export MIZE_MODULE_NO_EXTERNALS=1
export MIZE_CONFIG=$MIZE_CONFIG:module_dir.mme=/home/me/work/modules/modules/mme/dist
#export MIZE_CONFIG=$MIZE_CONFIG:module_dir.mme=/home/me/work/mize/result


# build mme for the browser
RUST_LOG=off wasm-pack build --dev --features wasm-target --no-default-features


# build mize for the browser
cd /home/me/work/mize
RUST_LOG=off wasm-pack build --target bundler --dev --features wasm-target --no-default-features #--out-dir ~/work/mize/src/platform/wasm/npm_pkg/generated
#export CARGO_BUILD_RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals'
#wasm-pack build --target no-modules --dev . -Z build-std=panic_abort,std --features wasm-target --no-default-features --config 'build.rustflags = "-C target-feature=+atomics,+bulk-memory,+mutable-globals"'
cp -r ~/work/mize/pkg/* ~/work/mize/src/platform/wasm/npm_pkg/generated
#RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' cargo build --target wasm32-unknown-unknown -Z build-std=panic_abort,std --lib --no-default-features
#cp ~/work/mize/target/wasm32-unknown-unknown/debug/mize.wasm ~/work/mize/src/platform/wasm/npm_pkg/generated/mize_bg.wasm

# build the mme-js presenter
cd /home/me/work/mme-presenters/presenters/mme-js/
npm run build -- --mode development


# build the mme module
cd /home/me/work/modules/modules/mme
cargo build --lib
mkdist

cd /home/me/work/modules/modules/mme/src/implementors/html/js-runtime
npm run build
cp -r dist/* /home/me/work/modules/mme/dist/js-runtime


# build the String module
cargo build --manifest-path ~/work/modules/modules/String/Cargo.toml --lib


# run mize with gui
cargo run --manifest-path ~/work/mize/Cargo.toml -- gui
