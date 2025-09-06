set -e

PROJECT_PATH="/home/elad/dev/rust-projects/ascii-player"
RUST_BIN="$PROJECT_PATH/target/release/ascii-player"
VIDEO_LINK="https://www.youtube.com/watch?v=TC80uw4HgCw"
NAME="teto"
HEIGHT=50
WIDTH=89

cargo build --release

/mnt/c/Program\ Files/Alacritty/alacritty.exe \
    -o window.dimensions.columns=$WIDTH \
    -o window.dimensions.lines=$HEIGHT \
    -o window.position.x=400 \
    -o window.position.y=200 \
    -o window.padding.x=0 \
    -o window.padding.y=0 \
    -o window.decorations=none \
    -e wsl "bash -l -c \"cd $PROJECT_PATH; $RUST_BIN --url $VIDEO_LINK --name $NAME\"; echo; echo Press enter to exit; read"
    #-e wsl "$RUST_BIN" "--url" "$VIDEO_LINK" "--name" "$NAME"