# Kierancli documentation.

Kierancli uses a mode system for accessing different functionality.
The command "mode x" allows switching between different modes. currently there are 3 modes:
rust (default), program and gitclone.

## Rust mode.
This mode allows execution of hard-coded systems such as kierancli's pwd, help, etc.

## Gitclone mode.
This mode is my scuffed git clone function using git2.
Entering "self" as the URL will clone kierancli's repository,
theoretically this could be used as a self-updater, however for updating I use this bash alias

alias kierancli-update="rm -rf ~/coding/rust/kierancli; git clone https://github.com/kierancrossland/kierancli ~/coding/rust/kierancli; cd ~/coding/rust/kierancli; cargo build --release"

Make a symlink from /usr/bin/kierancli -> ~/coding/rust/kierancli/release/kierancli
Now running kierancli-update will update and install the program! (be warned rust is slow to compile)

## Program mode.
This mode is a basic shell for running system programs eg: ls, cat neofetch.
Program mode works much better on Unix than Windows due some missunderstood PowerShell retardedness.
For example, opening another program like vim and exiting will cause kierancli to quit in PowerShell,
however Bash will keep running and return to program mode.

## Resources used.
https://endler.dev/2018/ls/
https://www.joshmcguigan.com/blog/build-your-own-shell-rust/
