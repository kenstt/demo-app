#!/bin/bash
# Ask the user for their name
echo Run Options:
echo 1: [core] demo play
echo 2: [core] unit test
read VAR

if [[ $VAR -eq 1 ]]
then
  cargo watch -q -c -w ./core  -x 'run -p core --bin play'
  elif [[ $VAR -eq 2 ]]
  then
  cargo watch -q -c -w ./core -x 'test -p core'
fi
