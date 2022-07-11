#!/bin/sh
#Information section
clear
echo "Welcome to the Unix installer for kierancli."
sleep 3
echo "Information can be found @ ~/.local/share/kieran_crossland/kierancli/Cargo.toml"
#Install section
echo BEGINNING INSTALL
cd ~/.local/share/
pwd
sleep 1
echo "Deleting prior versions in ~/.local/share/"
rm -rf ~/.local/share/kierancli/
rm -rf ~/.local/share/kieran_crossland/kierancli/
git clone https://github.com/kierancrossland/kierancli ~/.local/share/kieran_crossland/kierancli/
cd ~/.local/share/kieran_crossland/kierancli
echo "If install fails here then Cargo is not presesnt."
echo "Building with cargo --release"
cargo build --release
echo attempting to symlink binary to /usr/local/bin/
echo symlink could fail on other Unix systems due to their lack of sudo
sudo ln -sf ~/.local/share/kieran_crossland/kierancli/target/release/kierancli /usr/local/bin/kierancli
date > build_date
echo "Last build/install date @ ~/.local/share/kieran_crossland/kierancli/build_date"
which kierancli
echo "Ignore all compiler warnings, they are either your fault (fix them) or my fault for using depreciated functions. I love my depreciated functions and probably will keep using them."
echo Program installed, run kierancli.