echo "Loader my lib version"

cd ./ulib/mocklibc_lib/
make
cd -
cd ./batch_apps/
make
cd -
./loader_lib.sh
