# Mookaite

Mookaite is a script to map background images to x11 virtual desktops for the time being it requires the 'feh' command.

If your currous 'mookaite' is a type of gemstone.

I was unsatisfied with the other availible options because
 1. No way to map different images to different desktops.
 2. No 'random mode' meaning i couldn't find a tool that just picked a random
 image every time the virtual desktop is changed.
 3. I was unable to find a tool that would change background images continously(say every 5 minutes or so).

# Build
    cargo build --release

# Modes
  - mapped - assign each desktop a background and switch between them.

  - random - Change the background to a random image everytime the virtual desktop is changed.
