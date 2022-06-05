# Kierancli

Kierancli uses a mode system (similar to VIM) for accessing different functionality. eg: gitclone, commandinput, runprogram.

## Commandinput
This mode allows execution of hard-coded functionality such as kierancli's pwd, help, etc.

## Gitclone
This mode is my scuffed git clone function using git2.
Entering "self" as the URL will clone this repository, 
this could theoretically be used as a self-updater since the programs presumably not in any package manager. 

## Runprogram
This mode is a basic shell for running system programs eg: ls, cat neofetch. 
https://www.joshmcguigan.com/blog/build-your-own-shell-rust/
