#!/bin/bash
# Ask the user for their name
echo Run Options:
echo 1: [core] demo play
echo 2: [core] unit test
echo 3: [web] run web api server
echo 4: [service] unit test
echo 5: [web] unit test
echo 6: [tauri] dev
echo 7: [web] run web api server with HTTPS
echo 8: [wasm] build wasm
echo 9: [web] run web api server with gRPC
read VAR

if [[ $VAR -eq 1 ]]
  then
  cargo watch -q -c -w ./core  -x 'run -p core --bin play'
elif [[ $VAR -eq 2 ]]
  then
  cargo watch -q -c -w ./core -x 'test -p core'
elif [[ $VAR -eq 3 ]]
  then
  cargo watch -q -c -w ./web -w ./service -w ./core -x 'run -p web --bin web'
elif [[ $VAR -eq 4 ]]
  then
  cargo watch -q -c -w ./service -w ./core -x 'test -p service'
elif [[ $VAR -eq 5 ]]
  then
  cargo watch -q -c -w ./web -w ./service -w ./core -x 'test -p web'
elif [[ $VAR -eq 6 ]]
  then
  cargo tauri dev -- -p app
elif [[ $VAR -eq 7 ]]
  then
  cargo watch -q -c -w ./web -w ./service -w ./core -x 'run -p web --bin https'
elif [[ $VAR -eq 8 ]]
  then
  cargo watch -q -c -w ./wasm/ -w ./service/ -w ./core/ -x 'build -p wasm && wasm-pack build --target web ./wasm'
elif [[ $VAR -eq 9 ]]
  then
  cargo watch -q -c -w ./web/ -w ./service/ -w ./core/ -x 'run -p web --bin grpc'
fi
