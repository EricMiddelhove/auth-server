# Trainingsapp

This repository contains code for my App, which should be capable of managing training plans, keeping track of excercises and weights used in regular workouts. 

This repository solely the API accessing the database.
---
# Implemented Security Measures
The implemented Measures:

## Authentication and Authorisation
- This server is used as an authentication server. Everyone is authorised to create an account. Only the configured server knowing the shared secret is authorised to verify an auth token, requests trying to verify that do not contain the correct secret are not processed. 

## Transport Layer Security
- This Server is currently not deployed. Therefore no TLS Setup is in place. 
- On deployment HTTPS should be considered as a mandatory requirement for deployment. The Authorisation is using HTTP Headers and HTTP Bodies to exchange secrets. Having those encrypted is essential for the security of this system.

## Network security
- On deployment a strict firewall is needed. Default should be to deny traffic to every port.
- Only expose the API port. (With the current compose configuration port 443) Do not expose the DB Port (Currently 27017)

## Hardware Security
- On deployment an encrypted Hard drive should be selected

## Password Handling
- Passwords are saved using state of the art hashing and key derivation functions (Argon2).
- Even in memory clear text passwords are discarded as soon as possible.
- Using rust ensures that no pointers are left on freed data. 

- Bits of entropy of the password gets calculated on the register request of the user. Ensuring safe passwords are used without limiting the statespace of passwords by requiring characters.

## Still to implement features before going to production
### Secrets
- The Auth server and this server authenticate themselves via a shared secret. A Key Vault which is responsible for regularly swapping keys and storing them securely should be implemented

### Logging
- Currently the logging is not existent. A proper way of logging requests as well as errors should be implemented

### Database !IMPORTANT!
- Currently the database uses no log in. This has to be changed
- Currently the database is secured by not exposing it. But that is definitely a point that could fail easily by misconfiguring the deployment server accidentaly

### Passwords
- Also a dictionary attack should be performed by the server on the selected password making sure that the password not vulnerable even though it has high entropy.
- The way of calculating entropy has to be imporved to mitigate f64 rounding errors. 

---
# Creating a development environement
These instructions guide you through the setup of a development environment, enabling you to compile the code on your own machine and running it locally.

## Using Docker
Using docker you will run a pre compiled version of this repository. It is the quickest way get this repository running

1. Ensure to have docker and docker compose installed
2. Clone this repository and navigate to it in your terminal
3. Configure the environment variables in the compose.yaml file - If you are using this as a development setup the compose is configured correctly, if you are deploying, make sure to use safe secrets and safe passwords
4. Run `docker compose up`- This will download the [ericmiddelhove/erics-auth-server](https://hub.docker.com/r/ericmiddelhove/erics-auth-server) docker image as well as the required [mongod db](https://hub.docker.com/_/mongo) image.

## Using Cargo
Using cargo enables you to build this project yourself and run it locally.

1. Ensure to have the rust compiler and  cargo compiled. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone this repository and navigate to it in your terminal
3. Configure the environment variables - The setup-dev-environment.sh script will do that for you.
4. Run `cargo run` to start the compilation and execute the project.