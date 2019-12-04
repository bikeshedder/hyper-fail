#!/bin/bash

#DURATION=300
DURATION=1

WRK="wrk -d $DURATION -c 64 -t 4"
BASE_URL=http://localhost:3000

$WRK $BASE_URL/
