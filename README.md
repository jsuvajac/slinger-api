# Backend server for spell slinger
- User and Spell-Book storage
- Authentication via http-only session tokens after login
## Tools
- framework -> actix
- docker -> containers
- Redis -> session storage
- Db -> postgres

# `API`
## `/login`
### POST 
    Autenticate via email and passwd, returns an http-only session token and stores it in redis session storage
*-- requires that the account exists*
### `/logout`
### POST 
    Purge session cookie and redis session token
*-- requires authentication*
## `/user`
### PUT 
    Create new user
*-- does not require authentication*
### POST
    Update passwd
*-- requires authentication*
### DELETE
    Purges session (cookie and redis) and deletes the user
TODO: remove all spell books 

*-- requires authentication*
## `/spellbook`
*-- all require authentication*
### GET
    Get all spell books of current user
### PUT
    Create new spell book for existing user
### POST
    Update content of a spell book
### DELETE
    delete spell book
