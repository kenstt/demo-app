#!/bin/bash
# Ask the user for their name
echo Run Options:
echo 1: [core] demo play
echo 2: [core] unit test
echo 3: [web] run web api server
read VAR

if [[ $VAR -eq 1 ]]
  then
  cargo watch -q -c -w ./core  -x 'run -p core --bin play'
elif [[ $VAR -eq 2 ]]
  then
  cargo watch -q -c -w ./core -x 'test -p core'
elif [[ $VAR -eq 3 ]]
  then
  cargo watch -q -c -w ./web -w ./service -w ./core -x 'run -p web'
fi
