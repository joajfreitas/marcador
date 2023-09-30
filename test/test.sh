#!/bin/bash
set -e
clean_up () {
  rm -rf /root/.local/share/marcador
} 

trap clean_up EXIT

function assert {
  name=$1
  result=$2
  expected=$3
  if test "$result" = "$expected"
  then 
    echo "✅ $name"
  else 
    echo "result: $result"
    echo "expected: $expected"
    exit 1
  fi
}



#pacman -Syyu --noconfirm
#pacman -S python-pip --noconfirm

cd /marcador

#pip install --break-system-packages .

rm -rf /root/.local/share/marcador

assert "empty_bookmarks"  "$(marcador bookmarks)"  "" 

marcador add "example.com" "example" "example"
assert "non_empty_bookmarks"  "$(marcador bookmarks)"  "example.com" 

marcador server --port 6003 --root "/" &
pid=$!

sleep 1

assert "marcador_server_bookmarks" "$(marcador bookmarks --hostname http://127.0.0.1:6003)" "example.com"

kill -9 $pid

marcador server --port 6003 --root "/marcador" &
pid=$!

sleep 1

assert "marcador_server_bookmarks" "$(marcador bookmarks --hostname http://127.0.0.1:6003/marcador)" "example.com"

kill -9 $pid

rm -rf /root/.local/share/marcador
