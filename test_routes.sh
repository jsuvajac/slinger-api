#!/bin/bash

curl -H "Authorization: Bearer test-token" localhost:4000/user
echo -e '\n\n\n'
curl -v -H "Authorization: Bearer test-token" -H "Content-Type: application/json"  -X PUT -d '{"passwd": "usr123", "email": "foo1@bars.com"}' localhost:4000/user
echo -e '\n\n\n'
curl -H "Authorization: Bearer test-token" localhost:4000/user

echo -e '\n\n\n'
curl -v -H "Authorization: Bearer test-token" -H "Content-Type: application/json"  -X POST -d '{"passwd": "123", "email": "foo1@bars.com"}' localhost:4000/user
echo -e '\n\n\n'
curl -H "Authorization: Bearer test-token" localhost:4000/user

echo -e '\n\n\n'
curl -v -H "Authorization: Bearer test-token" -H "Content-Type: application/json"  -X DELETE -d '{"passwd": "usr123", "email": "foo1@bars.com"}' localhost:4000/user
echo -e '\n\n\n'
curl -H "Authorization: Bearer test-token" localhost:4000/user
