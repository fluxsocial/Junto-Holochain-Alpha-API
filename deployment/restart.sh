#!/bin/bash
supervisorctl stop conductor #Stop currently running conductor

rm -rf /home/josh/holochain/junto/storage/*
rm -rf /home/josh/holochain/n3h/*
echo "Deleted old storage directories"

echo "Running conductor generator"
/home/josh/Holochain-Conductor-Agent-Generator/target/release/conductor_generator \
    --agents 10 \
    --dna_hashs QmayTdQeSJWS3ftzfGQpame86xxMDekDYTwGixCfcngwUD \
    --dna_ids junto \
    --dna_paths /home/josh/Junto/junto-rust/dist/junto-rust.dna.json \
    --path /home/josh/holochain

supervisorctl start conductor #Start conductor back up - should now use fresh storage and networking states