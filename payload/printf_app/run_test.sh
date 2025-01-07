#!/bin/bash

# pthread_app：目前未测试对支持静态链接的支持情况

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
    python $CURRENT/build.py printf
    exit 0
elif [ "$TTYPE" = "static" ]; then
    echo "Static linking has not been tested yet."
    exit 1
else
    echo "Test failed"
    exit 1
fi