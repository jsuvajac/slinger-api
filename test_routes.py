import requests
import json
from pprint import pprint

url='https://localhost:4000'
session = requests.Session()

def print_req(route, req):
    print(f'{route}\nreq:{req.request.headers}\nresp:{req.headers}\nrext:{req.text}\n{req.cookies}\n')

# r = requests.put(url+'/user', json={'passwd': 'usr123', 'email': 'foo1@bars.com'}, verify=False)
# print_req('create user', r)

r = session.post(url+'/login', json={'passwd': 'usr123', 'email': 'foo1@bars.com'}, verify=False)
print_req('login', r)

# r = session.post(url+'/user', json={'passwd': 'usr123'}, verify=False)
# print_req('update user pass', r)

r = session.put(url+'/spellbook', json={'name': 'book5', 'content': '!@#!@#'}, verify=False)
print_req('create_book', r)

# r = session.delete(url+'/user', json={'email': 'foo1@bars.com'}, verify=False)
# print_req('delete user', r)

r = session.post(url+'/logout', verify=False)
print_req('logout', r)
