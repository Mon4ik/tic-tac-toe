ffmpeg -i showcase.mp4 \
    -vf "fps=10,scale=1200:-1" \
    -loop 0 showcase.gif