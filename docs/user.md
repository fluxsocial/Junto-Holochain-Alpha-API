# User/Auth Documentation

**Register**
###### Request: 
```
Endpoint: /register
Method: POST
Headers: Content-Type: application/json
Arguments: {"email": "..", "password": "..", "first_name": "..", "last_name": ".."}
```

###### Response Success: 
```
Status Code: 200
Body: {"user_id": ".."}
```

**Login**
###### Request: 
```
Endpoint: /auth
Method: POST
Headers: Content-Type: application/json
Arguments: {"email": "..", "password": ".."}
```

###### Response: 
```
Status Code: 200
Success-Body: none
Headers: set-cookie: auth=cookie; HttpOnly; Path=..; Domain=..; Max-Age=..
```

**Other Notes**
Making a post request to /register will not create a holochain-junto user for you. That must be done in a subsequent request to /holochain. 