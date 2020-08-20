#!/bin/bash

curl -k  \
    -H "Content-Type: application/json" -X PUT \
    -d '{"passwd": "usr123", "email": "foo1@bars.com"}' \
    https://localhost:4000/user

curl -k -H "Authorization: Bearer test-token" https://localhost:4000/user

echo -e '\n'

curl -k -v -c cookiejar.txt \
    -H "Content-Type: application/json" -X POST \
    -d '{"passwd": "usr123", "email": "foo1@bars.com"}' \
    https://localhost:4000/login

curl -k -v -b cookiejar.txt -X POST https://localhost:4000/logout
echo -e '\n'
curl -k -b cookiejar.txt  -H "Authorization: Bearer test-token" \
    -H "Content-Type: application/json" -X POST \
    -d '{"passwd": "111", "email": "foo1@bars.com"}' \
    https://localhost:4000/user
curl -k -b cookiejar.txt -H "Authorization: Bearer test-token" https://localhost:4000/user

echo -e '\n'

curl -k  -b cookiejar.txt -H "Authorization: Bearer test-token" \
    -H "Content-Type: application/json" -X POST \
    -d '{"passwd": "222", "email": "foo1@bars.com"}' \
    https://localhost:4000/user
curl -k -b cookiejar.txt -H "Authorization: Bearer test-token" https://localhost:4000/user

echo -e '\n'


curl -k  -b cookiejar.txt -H "Authorization: Bearer test-token" \
    -H "Content-Type: application/json" -X DELETE \
    -d '{"passwd": "usr123", "email": "foo1@bars.com"}' \
    https://localhost:4000/user

curl -k -b cookiejar.txt -H "Authorization: Bearer test-token" https://localhost:4000/user
