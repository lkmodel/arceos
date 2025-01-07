#!/bin/bash

TTYPE="dynamic"

# Parse arguments
for arg in "$@"; do
    case $arg in
        TTYPE=*)
            TTYPE="${arg#*=}"
            ;;
    esac
done

CURRENT=$PWD

echo "Test type: $TTYPE"
echo "Current path: $CURRENT"

if [ "$TTYPE" = "dynamic" ]; then
    echo "Building and running dynamic test"
    python $CURRENT/build.py $TTYPE
    exit 0
elif [ "$TTYPE" = "static" ]; then
    echo "Building and running dynamic test"
    python $CURRENT/build.py $TTYPE
    exit 0
else
    echo "Test failed"
    exit 1
fi


