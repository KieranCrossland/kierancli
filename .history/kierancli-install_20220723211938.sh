#!/bin/sh
#Information section
clear
echo "Welcome to the Shell installer for kierancli."
sleep 2
echo ""
echo "Information @ ~/.local/share/kierancrossland/kierancli/Cargo.toml"
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
rm -rf /etc/kierancrossland/
rm -rf
rm -rf ~/.local/share/kierancrossland/kierancli/
git clone https://github.com/kierancrossland/kierancli ~/.local/share/kierancrossland/kierancli/
cd ~/.local/share/kierancrossland/kierancli
echo ""
echo "If install fails here then Cargo is not presesnt."
echo ""
echo "Building with cargo --release"
echo ""
cargo build --release
echo ""
echo attempting to symlink binary to /usr/local/bin/
echo symlink could fail on other systems due to lack of sudo
sudo ln -sf ~/.local/share/kierancrossland/kierancli/target/release/kierancli /usr/local/bin/kierancli
sudo mkdir /etc/kierancrossland/
sudo mkdir /etc/kierancrossland/kierancli
sudo date > /etc/kierancrossland/kierancli/built
echo ""
echo "Last build/install date @ /etc/kierancrossland/kierancli/built"
echo ""
echo "Ignore all compiler warnings, they are either your fault (fix them) or my fault for using depreciated functions. I love my depreciated functions and probably will keep using them."

echo ""
which kierancli
echo ""
echo Program installed, run kierancli.
