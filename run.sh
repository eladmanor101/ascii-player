#!/usr/bin/env bash
set -e

RUST_BIN="/home/elad/dev/rust-projects/bad-apple-ascii/target/release/bad-apple-ascii"

# Read config file
VIDEO_TITLE=$(grep "^video_title" config.toml | cut -d '=' -f2 | tr -d ' ')
WIDTH=$(grep "^width" config.toml | cut -d '=' -f2 | tr -d ' ')
FPS=$(grep "^fps" config.toml | cut -d '=' -f2 | tr -d ' ')

generate_metadata() {
    jq -n \
        --arg video_title "$VIDEO_TITLE" \
        --argjson width "$WIDTH" \
        --argjson fps "$FPS" \
        '{video_title:$video_title, width:$width, fps:$fps}' \
        > assets/frames/metadata.json
}

generate_frames() {
    echo "Generating frames..."

    rm -rf assets/frames
    mkdir assets/frames
    ffmpeg -loglevel error -i assets/${VIDEO_TITLE}.mp4 -vf "fps=${FPS},scale=${WIDTH}:-1:flags=lanczos,crop=iw:ih/2" assets/frames/frame_%d.png

    echo "Done generating frames"

    # Cache result for future use
    generate_metadata
}

if [ -f assets/frames/metadata.json ]; then
    META_VIDEO_TITLE=$(jq -r '.video_title' assets/frames/metadata.json)
    META_WIDTH=$(jq '.width' assets/frames/metadata.json)
    META_FPS=$(jq '.fps' assets/frames/metadata.json)

    if [ "$META_VIDEO_TITLE" = "$VIDEO_TITLE" ] &&
        [ "$META_WIDTH" -eq "$WIDTH" ] &&
        [ "$META_FPS" -eq "$FPS" ]; then
        echo "Metadata matches, skipping frame generation"
    else
        echo "Metadata mismatch"
        generate_frames
    fi
else
    echo "No metadata"
    generate_frames
fi

# Detect width & height from one frame
HEIGHT=$(identify -format "%h" assets/frames/frame_1.png)
WIDTH=$(identify -format "%w" assets/frames/frame_1.png)

cargo build --release
cargo build

/mnt/c/Program\ Files/Alacritty/alacritty.exe \
    -o window.dimensions.columns=$WIDTH \
    -o window.dimensions.lines=$HEIGHT \
    -o window.position.x=400 \
    -o window.position.y=200 \
    -o window.padding.x=0 \
    -o window.padding.y=0 \
    -o window.decorations=none \
    -e wsl "$RUST_BIN" "$HEIGHT" "$WIDTH" "$FPS"