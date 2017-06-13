#!/bin/sh

# BASED ON: https://gist.github.com/mnlcandelaria/c9a741801caf15de4e8b

# run in sudo
# Before installing, make sure you have installed all the needed packages
sudo apt-get install libtool pkg-config build-essential autoconf automake
sudo apt-get install libzmq-dev

# Get libsodium
sudo add-apt-repository ppa:chris-lea/libsodium;
sudo echo "deb http://ppa.launchpad.net/chris-lea/libsodium/ubuntu trusty main" >> /etc/apt/sources.list;
sudo echo "deb-src http://ppa.launchpad.net/chris-lea/libsodium/ubuntu trusty main" >> /etc/apt/sources.list;
sudo apt-get update && sudo apt-get install libsodium-dev;

# Install zeromq
# latest version as of this post is 4.1.2
wget http://download.zeromq.org/zeromq-4.1.2.tar.gz
tar -xvf zeromq-4.1.2.tar.gz
cd zeromq-4.1.2
./configure
make
sudo make install
