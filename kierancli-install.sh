#!/bin/sh
#Information section
clear
echo "Welcome to the Unix installer for kierancli."
sleep 2
echo ""
echo "Information @ ~/.local/share/kieran_crossland/kierancli/Cargo.toml"
echo ""
echo "Git repository @ https://github.com/KieranCrossland/kierancli/"
#Install section
echo ""
cd ~/.local/share/
sleep 1
echo "Configuring directories in:"
pwd
echo ""
rm -rf ~/.local/share/kierancli/
rm -rf ~/.local/share/kierancrossland/kierancli/
git clone https://github.com/kierancrossland/kierancli ~/.local/share/kierancrossland/kierancli/
cd ~/.local/share/kierancrossland/kierancli
echo ""
echo "If install fails here then Cargo is not presesnt."
echo ""
echo "Building with cargo --release"
echo ""
cargo build --release
echo attempting to symlink binary to /usr/local/bin/
echo symlink could fail on other Unix systems due to their lack of sudo
sudo ln -sf ~/.local/share/kierancrossland/kierancli/target/release/kierancli /usr/local/bin/kierancli
date > build_date
echo ""
echo "Last build/install date @ ~/.local/share/kierancrossland/kierancli/build_date"
echo ""
echo "Ignore all compiler warnings, they are either your fault (fix them) or my fault for using depreciated functions. I love my depreciated functions and probably will keep using them."

echo ""
which kierancli
echo ""
echo Program installed, run kierancli.
