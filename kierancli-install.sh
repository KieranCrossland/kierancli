#!/bin/sh
#Information section
clear
echo ""
echo "Welcome to the Unix installer for kierancli."
sleep 3
echo ""
echo "SOFTWARE INFORMATION"
sleep 1
echo ""
echo "Contact email 'kierancrossland1st@gmail.com'" 
sleep 0.4
echo "License = 'GNU AGPL-3.0'"
sleep 0.4
echo "source repository = 'https://github.com/kierancrossland/kierancli/'"
echo ""
sleep 1
#Install section
echo BEGINNING INSTALL
echo ""
sleep 1
echo "Deleting prior versions in ~/.local/share/"
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
echo ""
echo attempting to symlink binary to /usr/local/bin/
echo ""
echo symlink could fail on other Unix systems due to their lack of sudo
sudo ln -sf ~/.local/share/kieran_crossland/kierancli/target/release/kierancli /usr/local/bin/kierancli
echo ""
pwd
date > build_date
echo "last build/install date @ ~/.local/share/kieran_crossland/kierancli/build_date"
echo ""
which kierancli
echo Program installed, run kierancli.