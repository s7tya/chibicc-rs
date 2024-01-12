cargo build 2> /dev/null
./target/debug/rs-9cc "$1" > tmp.s
cc -o tmp tmp.s
./tmp
echo $?