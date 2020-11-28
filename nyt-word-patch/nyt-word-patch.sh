#!/usr/bin/env bash

die () {
    echo >&2 "$@"
    exit 1
}

[ "$#" -eq 2 ] || die "2 arguments required, $# provided"
[ -e $1 ] || die "file $1 does not exist"
[ -e $2 ] || die "file $2 does not exist"

cat $2 | dos2unix | grep -Fvxf $1
