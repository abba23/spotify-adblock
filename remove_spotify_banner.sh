#!/bin/bash

# Script to automatically fix Spofity (after it has been updated)
# See: https://github.com/abba23/spotify-adblock/issues/12
# SPDX-License-Identifier: GPL-3.0-or-later

if which spotify > /dev/null; then
    sPATH=$(dirname "$(realpath -L "$(which spotify)")")  # absolute path to Spotify
elif flatpak list | grep --q com.spotify.Client; then
    sPATH="$(flatpak --installations)/app/com.spotify.Client/current/active/files/extra/share/spotify"
else
    echo "Spotify not found"
    exit 0
fi
cd "${sPATH}/Apps/" || exit 1

if [ "$1" == 'restore' ]; then
    if [ -w xpui.spa_bak ] && [ -w . ]; then
        rm -f xpui.spa
        mv xpui.spa_bak xpui.spa
        echo "Restore success"
    else
        [ -f xpui.spa_bak ] && echo "Permission denied" || echo "Backup not found"
    fi
    exit 0
fi

if [ -w xpui.spa ] && [ -w . ]; then
    cp xpui.spa xpui.spa_bak  # create a backup, in case of trouble
    # shellcheck disable=SC2094
    unzip -p xpui.spa xpui.js | sed 's/adsEnabled:\!0/adsEnabled:false/' > xpui.js
    zip --update xpui.spa xpui.js
    rm xpui.js
    echo "Success"
else
    [ -f xpui.spa ] && echo "Permission denied" || echo "File not found"
fi
