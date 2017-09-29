#!/bin/bash

source settings.sh

echo Building project...
cargo build --target=$TARGET

echo Syncing with PI...
rsync -ru --delete ./target/$TARGET pi@$IP:/home/pi/soulful-companion/target

echo Running on PI.
ssh pi@$IP "cd /home/pi/soulful-companion; target/$TARGET/debug/soulful-companion"