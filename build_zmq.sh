#!/bin/sh

# BASED ON: https://gist.github.com/mnlcandelaria/c9a741801caf15de4e8b

# run in sudo
# Before installing, make sure you have installed all the needed packages
sudo apt-get install libtool pkg-config build-essential autoconf automake
sudo apt-get install libzmq-dev

# Get libsodium
git clone git://github.com/jedisct1/libsodium.git
cd libsodium
./autogen.sh
./configure && make check
sudo make install
sudo ldconfig

cd ..

# Install zeromq
# latest version as of this post is 4.2.1
wget http://download.zeromq.org/zeromq-4.2.1.tar.gz
tar -xvf zeromq-4.2.1.tar.gz
cd zeromq-4.2.1
./configure
make
sudo make install
