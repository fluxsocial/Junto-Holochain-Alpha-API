#!/bin/bash

if [ -d "/holochain/dnas" ] 
then
    while [ "$1" ];
    do
        echo -e "\e[33mHolochain directory already exists"
        cd /holochain/dnas/Junto/junto-rust
        hc package
        cd /holochain/dnas/DeepKey
        hc package
        echo -e "\e[32mHolochain DNA and directories setup"
    done
else
    mkdir /holochain/keys
    mkdir /holochain/dnas 
    mkdir /holochain/dnas/Junto
    mkdir /holochain/dnas/DeepKey
    mkdir /holochain/persistence
    mkdir /holochain/junto
    mkdir /holochain/deepkey
    mkdir /holochain/junto/storage /holochain/junto/storage/host /holochain/junto/storage/config
    mkdir /holochain/deepkey/storage /holochain/deepkey/storage/host /holochain/deepkey/storage/config

    git clone https://github.com/juntofoundation/Junto.git /holochain/dnas/Junto
    git clone https://github.com/Holo-Host/DeepKey.git /holochain/dnas/DeepKey
    cd /holochain/dnas/Junto/junto-rust
    hc package
    cd /holochain/dnas/DeepKey
    hc package
    echo -e "\e[32mHolochain DNA and directories setup"
fi

holochain --config ~/Junto-Alpha-API/conductor-config.toml