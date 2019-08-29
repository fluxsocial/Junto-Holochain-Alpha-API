# Junto-Holochain-Alpha-API

This repository contains the code to run a HTTP wrapper around a given locally running holochain conductor. This allows the use of a holochain application in a centralized manor for the purposes of testing.

Due to the nature of holochain conductors it is impossible to add holochain agent/instances to the conductor without causing the conductor to restart. This is problematic in a multi-user scenario as it means requests may go unheard if a new user wishes to sign up.

Instead we pre-generate agents and thus a conductor config file and assign API users to free agents as users request to join the application. This negates the need for the restarting of the conductor and allows many users to test a holochain application and its operation.

This web app uses basic cookie based authentification to identify requests with agents. One endpoint is used to interface with the holochain application. All data is passed in and out of the holochain application as if you were running a HTTP holochain conductor locally. More information can be found in the documentation below.

## [Web-App Documentation](https://github.com/juntofoundation/Junto-Holochain-Alpha-API/docs)
## [Conductor Generator](https://github.com/juntofoundation/Junto-Holochain-Alpha-API/tree/development/conductor_generator)