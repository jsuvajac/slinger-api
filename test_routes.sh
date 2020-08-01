#!/bin/bash

curl localhost:4000/user
echo -e '\n\n\n'
curl -v -H "Content-Type: application/json"  -X PUT -d '{"passwd": "usr123", "email": "foo1@bars.com"}' localhost:4000/user
echo -e '\n\n\n'
curl localhost:4000/user

echo -e '\n\n\n'
curl -v -H "Content-Type: application/json"  -X POST -d '{"passwd": "123", "email": "foo1@bars.com"}' localhost:4000/user
echo -e '\n\n\n'
curl localhost:4000/user

echo -e '\n\n\n'
curl -v -H "Content-Type: application/json"  -X DELETE -d '{"passwd": "usr123", "email": "foo1@bars.com"}' localhost:4000/user
echo -e '\n\n\n'
curl localhost:4000/user
