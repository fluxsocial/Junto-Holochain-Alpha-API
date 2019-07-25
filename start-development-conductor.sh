#!/bin/bash

if [ -d "/holochain" ] 
then
    echo "Holochain directory already exists"
else
    mkdir /holochain
    mkdir /holochain/dnas 
    mkdir /holochain/persistence
    mkdir /holochain/junto
    mkdir /holochain/deepkey
    touch /holochain/junto/storage
    touch /holochain/deepkey/storage

    git clone https://github.com/juntofoundation/Junto.git /holochain/dnas
    git clone https://github.com/Holo-Host/DeepKey.git /holochain/dnas
    cd /holochain/dnas/Junto
    hc package
    cd /holochain/dnas/DeepKey
    hc package
    echo "\e[32mHolochain DNA and directories setup"
fi

holochain --config ~./Junto-Alpha-API/conductor-config.toml