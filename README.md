# Mookaite 0.5

###### Mookaite is a script to map background images to x11 virtual desktops for the time being it requires the 'feh' command.

#### **Not quite ready for human consumption.**

If your currous 'mookaite' is a type of gemstone.

#### I was unsatisfied with the other availible options because
 1. No way to map different images to different desktops.

 2. No 'random mode' meaning i couldn't find a tool that just picked a random
 image every time the virtual desktop is changed.

 3. I was unable to find a tool that would change background images continously(say every 5 minutes or so).

 4. Couldn't find a tool that monitored the image directory for changes.

# Build
    cargo build --release


# Modes
  - mapped - assign each desktop a background and switch between them.

  - random - Change the background to a random image everytime the virtual desktop is changed.

# Running
  Run in deafult(my) mode.

  `./mookaite #mookaite -c /usr/bin/feh -m random -d /home/$USER/Pictures -t 300 -r 6000`

  Run in mapped mode, changing background every 10mins passing "--bg-tile" to feh,
  also sending logs 'straight to hell'.

  `./mookaite -m mapped -t 600 -l /dev/null -args --bg-tile`

  Run with xsetroot in random mode. Changing image every minute

  `./mookaite -c /usr/bin/xsetroot -l /home/$USER/.config/mookaite.log -t 60`

# Notes

  - If the background command is '/usr/bin/feh' and **NO** '--args' are given '--bg-scale' is still passed, this is simply to
    make my personal settings the default so i could be lazy. To circumvent this behavior
    pass an empty '--args' flag to remove it, **but this might cause feh to run in GUI mode**.

  - This program is only really intended for my own personal use but if someone
    started using it as well, i'd be willing to add or change features.
