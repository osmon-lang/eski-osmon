#!/bin/sh

set -e

# Test that we can install the latest version at the default location.
rm -f ~/.osmon/bin/osmon
unset OSMON_INSTALL
sh ./install.sh
~/.osmon/bin/osmon --version

# Test that we can install a specific version at a custom locations.
rm -rf ~/osmon-0.4.0
export OSMON_INSTALL="$HOME/osmon-0.4.0"
./install.sh v0.4.0
~/osmon-0.4.0/bin/osmon --version | grep 0.4.0

# Test that we can install at a relative custom location.
export OSMON_INSTALL="."
./install.sh v0.4.0
bin/osmon --version | grep 0.4.0
