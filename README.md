# Sound Machine
A small utility, to help convert a rasberry pi, into a sound machine.  

_Helping me get sleep, by helping my children stay asleep._  

## Installation

The following steps will cause the sound machine to run, whenever you launch your raspberry pi.

First clone this repository, and install the sound machine binary using the `cargo install` command.

```bash
cargo install --path .
```

Next run the install script to setup systemd to launch this binary on startup.

```bash
sh install.sh
```

### Troubleshooting
Some common issues, and fixes I found, setting up the dev environment.
- Revieve build error about not being able to find the `alsa` library, `alsa was not found in the pkg-config search path`. This is likely, because you are missing the `libsound2-dev` package.
    + The fix is to install with `sudo apt install libasound2-dev`.
