#!/bin/sh
reduce-binsrch '\[(?:[^\[\]]+|\[[^\w\[\]]*\])*\]' $1 ''
