#!/bin/sh
cp $PWD/target/release/i3blocks_date $PWD/rootdir/usr/localbin/i3blocks_date
pkg create -m $PWD/metadata -o $PWD/pkg-builds -p $PWD/metadata/PLIST -r $PWD/rootdir
