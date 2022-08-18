#!/usr/bin/env bash

# Usage: whichMan.sh 'some string that may be in some package's manpage'

# Why ?
#
# Because sometimes you don't know which manpage to consult
# to get info about something.
#
# This tools greps all the man-pages for that something,
# then show you excerpts of what it found and helps you
# decide which man-page to consult to get info about
# that particular subject.
#
# man -K does this, more or less, but makes you cycle through each
# man-page manually and doesn't provide you with a good overview.
# This script aims to correct that.
#
# Doesn't seem to work with bash, for some reason I have not yet investigated,
# so currently it is zsh-only - the best shell anyway.
#
# Why the unreadable one-liner ?
# For the fun of it, to level-up in shell scripting.
# I know this tool could have been written in a way more
# readable way, and I will do it too, eventually.

(export QUERY=$1; \
man -waK $QUERY | grep '.gz' | sed -r 's/^([^.]+)\..*$/\1/' | \
xargs -n1 basename | xargs -n1 -I% sh -c \
'{ echo "\n[####] \033[0;32mFound match in the man page of < % >:\033[0m\n"; \
man % | grep "$QUERY" -i -A2 -B2 --color=always; echo "\n"; }')
