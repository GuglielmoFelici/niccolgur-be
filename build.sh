TARGET="armv7-unknown-linux-gnueabihf"
#$TARGET=aarch64-unknown-linux-gnu
COMPILER="arm-linux-gnueabihf-gcc"
rustup target add armv7-unknown-linux-gnueabihf 1>&/dev/null
if ! command -v $COMPILER  &> /dev/null; then
    echo "E' necessario installare $COMPILER"
    exit 1
fi
if ! grep -q "target.armv7-unknown-linux-gnueabihf" ~/.cargo/config; then
    echo "E' necessario settare linker = arm-linux-gnueabihf-gcc per il target target.armv7-unknown-linux-gnueabihf in ~/.cargo/config"
    exit 2
fi

cargo build --release --target=$TARGET
