# Sound Machine
A small utility, to help convert a rasberry pi, into a sound machine.  

_Helping me get sleep, by helping my children stay asleep._  

### Troubleshooting
Some common issues, and fixes I found, setting up the dev environment.
- Revieve build error about not being able to find the `alsa` library, `alsa was not found in the pkg-config search path`. This is likely, because you are missing the `libsound2-dev` package.
    + The fix is to install with `sudo apt install libasound2-dev`.
