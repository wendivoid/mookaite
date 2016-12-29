# Mookaite 0.5

Mookaite is a script to map background images to x11 virtual desktops for the time being it requires the 'feh' command.
**Not quite ready for human consumption.**
If your currous 'mookaite' is a type of gemstone.

I was unsatisfied with the other availible options because
 1. No way to map different images to different desktops.
 2. No 'random mode' meaning i couldn't find a tool that just picked a random
 image every time the virtual desktop is changed.
 3. I was unable to find a tool that would change background images continously(say every 5 minutes or so).
 4. Couldn't find a tool that monitored the image directory for changes
 5.
# Build
    cargo build --release


# Modes
  - mapped - assign each desktop a background and switch between them.

  - random - Change the background to a random image everytime the virtual desktop is changed.

# Running
  Run in deafult(my) mode.

  `./mookaite #mookaite -m random -d /home/$USER/Pictures -t 300 -r 6000`

  Run in mapped mode, changing background every 10mins passing "--bg-tile" to feh,
  also sending logs 'straight to hell without passing'.

  `./mookaite -m mapped -t 600 -l /dev/null --feh-args --bg-tile`

# Notes

  - If no '--feh-args' are given '--bg-scale' is still passed, this was simply to
    make my personal settings the default so i could be lazy. The goal is to be able
    to pass an empty '--feh-args' flag to remove it, **but this is not implemented yet.**

  - Their is one more feature i **plan** on adding witch is being able to change 'feh'
    to whatever background setting command you wish to use. When that is complete i willing
    consider it version 0.9. After i've used it for a week or two and fixed any bug's that
    pop up i will release version 1.0.

  - I am by no means a lawyer so as soon as i figure out witch license translate to
    "idgaf what you do with it" i will license it.

  - With the above being said this readme is still acurate for the current version, and i
    will update it appropriatly.

  - This program is only really intended for my own personal use but if someone
    started using it as well, i'd be willing to add or change features.

  - My computer sucks, (1 Core, 1.7 mem) so a couple things i did to make mookaite
    a little more useful to me.

      1. Their are 2 pauses, 1 (500ms) each cycle and 1 (750ms) only if their were no
      x events.

      2. Set proccess 'nice' value (+10) using libc.
