#!/bin/sh
clear
echo "Welcome to the Shell installer for kierancli."
echo ""
echo "Information @ ~/.local/share/kierancrossland/kierancli/Cargo.toml"
echo "Git repository @ https://github.com/KieranCrossland/kierancli/"
echo ""
cd ~/.local/share/
echo "Configuring directories in:"
pwd
sudo rm -rf ~/.local/share/kierancli/
rm -rf ~/.local/share/kierancrossland/kierancli/
git clone https://github.com/kierancrossland/kierancli ~/.local/share/kierancrossland/kierancli/
cd ~/.local/share/kierancrossland/kierancli
echo "Cargo MUST be installed to build. If the install fails here, try setting your 'default' rustup toolchain."
cargo build --release
echo ""
echo "Symlinking binary to /usr/local/bin/"
echo "Symlinking requres sudo."
sudo ln -sf ~/.local/share/kierancrossland/kierancli/target/release/kierancli /usr/local/bin/kierancli
date >> datebuilt
echo ""
which kierancli
echo Program installed, run kierancli.
