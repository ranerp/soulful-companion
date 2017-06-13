#!/bin/bash

source settings.sh

cargo build --target=$TARGET
rsync -ru --delete ./target/$TARGET pi@$IP:/home/pi/soulful-companion/target
ssh pi@$IP "/home/pi/soulful-companion/target/$TARGET/debug/soulful-companion"