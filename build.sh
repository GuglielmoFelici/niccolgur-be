$TARGET=armv7-unknown-linux-gnueabihf
#$TARGET=aarch64-unknown-linux-gnu
$COMPILER=arm-linux-gnueabihf-gcc
rustup target add armv7-unknown-linux-gnueabihf
if ! command -v $COMPILER  &> /dev/null; then
    echo "E' necessario installare $COMPILER"
    exit
fi

cargo build --release --target=$TARGET
