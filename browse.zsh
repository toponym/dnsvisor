#!/bin/zsh
# Resolve a domain and get the page at the IP
# First start up dns resolver:
# cargo run server 127.0.0.1 1053
set -e
if [ $# -lt 1 ]; then
	echo "Usage: $0 <domain-name> [-t (ground truth)]"
	exit 1
fi
SITE=$1
FILE="webpage.html"
rm $FILE || true

shift

if [[ "$1" == "-t" ]]; then
	# use dig for ground truth
	IP=$(dig +short $SITE | tail -n 1)
else
	# use server
	IP=$(dig +short +noedns @127.0.0.1 -p 1053 $SITE | tail -n 1)
fi
echo "Using IP: $IP"
curl -L -o $FILE --resolve $SITE:80:$IP http://$SITE
open $FILE
