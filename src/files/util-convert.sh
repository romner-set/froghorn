ffmpeg -i frog.CUT.mp4 \
  -c:v libx264 -crf 23 -profile:v baseline -level 3.0 -pix_fmt yuv420p \
  -b:v 3000k \
  -movflags faststart \
  -vf scale=854:480 \
  -r 24 \
  frog.h264
