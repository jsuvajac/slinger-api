#!/bin/bash

curl -k  -H "Authorization: Bearer test-token" \
    -H "Content-Type: application/json" -X PUT \
    -d '{"passwd": "usr123", "email": "foo1@bars.com"}' \
    https://localhost:4000/user
curl -k -H "Authorization: Bearer test-token" https://localhost:4000/user

echo -e '\n\n'

curl -k  -H "Authorization: Bearer test-token" \
    -H "Content-Type: application/json" -X POST \
    -d '{"passwd": "123", "email": "foo1@bars.com"}' \
    https://localhost:4000/user
curl -k -H "Authorization: Bearer test-token" https://localhost:4000/user

echo -e '\n\n'

curl -k  -H "Authorization: Bearer test-token" \
    -H "Content-Type: application/json" -X DELETE \
    -d '{"passwd": "usr123", "email": "foo1@bars.com"}' \
    https://localhost:4000/user
curl -k -H "Authorization: Bearer test-token" https://localhost:4000/user
