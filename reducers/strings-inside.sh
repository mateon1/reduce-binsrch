#!/bin/sh
reduce-binsrch '(?P<double>")(?:[^\\"\n]|\\[\s\S])*"|(?P<single>'"')(?:[^\\\\'\\n]|\\\\[\\s\\S])*'"'' $1 '$double$double$single$single'
