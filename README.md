# Kierancli.

Kierancli uses a mode system for accessing different functionality.
The command "mode x" allows switching between different modes. currently there are 3 modes:
rust (default), program and gitclone.

## Installation / Updating.
``curl https://raw.githubusercontent.com/KieranCrossland/kierancli/master/kierancli-install.sh | sh``

On GNU/Linux the shell script kierancli-install.sh
will download, install, and update kierancli to its latest version on Git master.

The program will be installed to `~/.local/share/kierancli/bin`, however this location can be easily changed by editing the script.

This software may work on MacOS, BSD, REDOX, and other Unix systems. However it has only been tested on GNU/Linux and windows (windows is unstable).

## Rust mode.
This mode allows execution of kierancli's built in functions such as pwd, ls, help, etc.
Functionality in Rust mode is hard-coded into the binary, thus editing this requires recompilation.

## Gitclone mode.
This mode is a scuffed git clone function using git2.
Entering "self" as the URL will clone kierancli's repository. 

## Program mode.
This mode is a basic shell for running system programs eg: ls, cat, neofetch. It will attempt to execute anything that is in $PATH.

Program mode works much better on Linux than Windows due some missunderstood PowerShell retardedness.
For example, opening another program like vim and exiting will cause kierancli to quit in PowerShell,
however Bash will keep running and return to program mode.
Sometimes this behaviour doesn't occur and the windows version functions correctly, however for the best experience use a Unix system.

## Resources used.
https://endler.dev/2018/ls/

https://www.joshmcguigan.com/blog/build-your-own-shell-rust/
