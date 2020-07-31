#!/bin/bash

curl localhost:4000/users
curl -v -H "Content-Type: application/json"  -X POST -d '{"passwd": "usr123", "email": "foo1@bars.com"}' localhost:4000/users
curl localhost:4000/users
curl -v -H "Content-Type: application/json"  -X DELETE -d '{"passwd": "usr123", "email": "foo1@bars.com"}' localhost:4000/users
curl localhost:4000/users
