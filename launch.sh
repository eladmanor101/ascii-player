set -e

RUST_BIN="/home/elad/dev/rust-projects/ascii-player/target/release/ascii-player"
VIDEO_LINK="https://www.youtube.com/watch?v=TC80uw4HgCw"
NAME="teto"
HEIGHT=80
WIDTH=120

/mnt/c/Program\ Files/Alacritty/alacritty.exe \
    -o window.dimensions.columns=$WIDTH \
    -o window.dimensions.lines=$HEIGHT \
    -o window.position.x=400 \
    -o window.position.y=200 \
    -o window.padding.x=0 \
    -o window.padding.y=0 \
    -o window.decorations=none \
    -e wsl bash -c 'exec "$0" "$@" ; echo "Press Enter to exit..."; read' "$RUST_BIN" --url "$VIDEO_LINK" --name "$NAME"
#    -e wsl bash -c "$RUST_BIN --url \"$VIDEO_LINK\" --name \"$NAME\"; echo 'Press Enter to exit...'; read" dummy
#    -e wsl "$RUST_BIN" "--url" "$VIDEO_LINK" "--name" "$NAME"