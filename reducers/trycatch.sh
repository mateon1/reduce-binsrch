#!/bin/sh
reduce-binsrch 'try\s*\{([\s\S]*?)\}\s*catch\s*\(.*?\)\s*\{[^}]*\}' $1 '{$1}'
