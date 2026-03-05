#!/usr/bin/env bash

OUT_DIR=storage
YEAR=2026

for m in {1..12}; do
    for d in {1..28}; do
        date_str=$(printf "%s-%02d-%02d" "$YEAR" "$m" "$d")
        {
            for i in {1..20}; do
                printf "\"%s %02s:00:00\",\"entry %s\",\"\"\n" "$date_str" "$i" "$i"
            done
        } > "${OUT_DIR}/${date_str}.log"
    done
done
