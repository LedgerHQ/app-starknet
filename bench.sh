#!/bin/zsh

start=$(($(gdate +%s%0N)/1000000))
ledgercomm-send --hid file "$1"
end=$(($(gdate +%s%0N)/1000000))
elapsed=$((end-start))
echo "Elapsed time: $elapsed ms"
