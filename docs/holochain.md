# Holochain Conductor Request Documentation

To call functions on Holochain, you must first make a request to the endpoint `/holochain` and pass it the required json body as a string.
Functions must be called based on the `zome` they are found in. For example, to create a user on holo, you must first reference the `user` zome then pass the function name to the parameter `function`. Finally, the arguments required by the function should be passed to the `args` key.

**Call Holochain**
###### Request: 
```
Endpoint: /holochain
Method: POST
Headers: Content-Type: application/json
Arguments: {"args": "{holochain zome function arguments}", "zome": "zome-to-call", "function": "function-to-call"}
```

###### Response Success: 
```
Status Code: 200
Body: "{json data from holochain...}"
```

###### Response Error: 
```
Status Code: 200
Body: "{json error data from holochain...}"
```

**Other Notes**
In the backend this request information is passed directly into the holochain [conductor](https://developer.holochain.org/guide/latest/conductors.html). This means that all data passed to this endpoint should adhere to our holochain applications API [specification](https://github.com/juntofoundation/Junto/tree/master/junto-rust/docs). All responses will also be returned directly from the holochain conductor and will follow the same API specifications as our holochain application.
