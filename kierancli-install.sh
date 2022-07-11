#!/bin/sh
#Information section
echo "Welcome to the installer for kierancli."
sleep 2
echo "SOFTWARE INFORMATION"
sleep 0.2
echo ""
echo "Contact email 'kierancrossland1st@gmail.com'" 
sleep 0.2
echo "License = 'GNU AGPL-3.0'"
sleep 0.2
echo "source repository = 'https://github.com/kierancrossland/kierancli/'"
echo ""
echo ""

#Install section
echo BEGINNING INSTALL / UPGRADE
echo ""
sleep 0.5
echo "Deleting prior versions in ~/.local/share/"
rm -rf ~/.local/share/kierancli/
rm -rf ~/.local/share/kieran_crossland/kierancli/
git clone https://github.com/kierancrossland/kierancli ~/.local/share/kieran_crossland/kierancli/
cd ~/.local/share/kieran_crossland/kierancli
echo "If install fails here then Cargo is not presesnt."
echo "running cargo build --release"
cargo build --release
echo ""
echo attempting to symlink binary to /usr/local/bin/
echo symlink could fail on other Unix systems due to their lack of sudo
sudo ln -sf ~/.local/share/kieran_crossland/kierancli/target/release/kierancli /usr/local/bin/kierancli
echo ""
pwd
date > build_date
echo "last build/install date @ ~/.local/share/kieran_crossland/kierancli/build_date"
echo ""
which kierancli
echo Program installed, run kierancli.