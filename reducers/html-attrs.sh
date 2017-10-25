#!/bin/sh
reduce-binsrch '\b\w+=(?:"[^"\n]*"|'"'"'[^'"'"'\n]*'"'"'"|\S*)' $1
