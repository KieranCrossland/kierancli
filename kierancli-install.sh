#!/bin/sh
#Information section
clear
echo "Welcome to the Unix installer for kierancli."
sleep 2
echo "Information can be found @ ~/.local/share/kieran_crossland/kierancli/Cargo.toml"
#Install section
echo ""
cd ~/.local/share/
pwd
sleep 1
echo "Configuring directories in ~/.local/share/"
echo ""
rm -rf ~/.local/share/kierancli/
rm -rf ~/.local/share/kieran_crossland/kierancli/
git clone https://github.com/kierancrossland/kierancli ~/.local/share/kieran_crossland/kierancli/
cd ~/.local/share/kieran_crossland/kierancli
echo "If install fails here then Cargo is not presesnt."
echo ""
echo "Building with cargo --release"
echo ""
cargo build --release
echo attempting to symlink binary to /usr/local/bin/
echo symlink could fail on other Unix systems due to their lack of sudo
sudo ln -sf ~/.local/share/kieran_crossland/kierancli/target/release/kierancli /usr/local/bin/kierancli
date > build_date
echo ""
echo "Last build/install date @ ~/.local/share/kieran_crossland/kierancli/build_date"
echo ""
echo "Ignore all compiler warnings, they are either your fault (fix them) or my fault for using depreciated functions. I love my depreciated functions and probably will keep using them."

echo ""
which kierancli
echo ""
echo Program installed, run kierancli.
