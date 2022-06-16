#!/bin/sh
#Information section
echo SOFTWARE DETAILS
sleep 0.8
echo "name = 'kierancli'"
sleep 0.8
echo "authors = ['Kieran Crossland <kierancrossland1st@gmail.com>']" 
sleep 0.8
echo "readme = 'README.md'"
sleep 0.8
echo "license = 'AGPL-3.0'"
sleep 0.8
echo "repository = 'https://github.com/kierancrossland/kierancli/'"
sleep 0.8
echo ""
echo ""
#Install section
echo BEGINNING INSTALL / UPGRADE
echo ""
sleep 0.5
echo "running rm -rf ~/.local/share/kierancli/"
rm -rf ~/.local/share/kierancli/
git clone https://github.com/kierancrossland/kierancli ~/.local/share/kierancli/
cd ~/.local/share/kierancli
echo "If install fails here then Cargo is not presesnt."
echo "running cargo build --release"
cargo build --release
mkdir ~/.local/share/kierancli/bin
mv ~/.local/share/kierancli/target/release/kierancli ~/.local/share/kierancli/bin/
echo ""
echo attempting to symlink binary to /usr/local/bin/
echo symlink could fail on BSD/Unix due to lack of sudo
sudo ln -sf ~/.local/share/kierancli/bin/kierancli /usr/local/bin/kierancli
echo ""
echo INSTALL SUCCEEDED
date > build_date
echo "last build date @ ~/.local/share/kierancli/build_date"
echo ""
echo run kierancli