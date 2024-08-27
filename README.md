# Introduction

&nbsp;&nbsp;&nbsp;&nbsp;There are networks like [IBM World Community Grid](https://www.worldcommunitygrid.org/) where various institutions upload projects for which 
they need to perform simulations or tests on a large scale, requiring substantial computing power. 
Generally, these projects that distribute their calculations contribute to scientific work, so it is 
crucial that the results are verifiable, or one must trust in the results. Because of this, the types 
of problems to be solved should yield results that are easily validated by the user, setting 
aside potential programs that would have high computational complexity, making verification of multiple 
results unfeasible.


&nbsp;&nbsp;&nbsp;&nbsp;In the face of this problem, one approach is to use a computational integrity test generated by the 
Cairo virtual machine. This allows for addressing more complex problems with solutions whose verification 
is resource intensive. Instead of verifying the solution itself, it becomes possible to verify that the returned 
result comes from a correct execution of the virtual machine on the provided code.


&nbsp;&nbsp;&nbsp;&nbsp;The objective of this project is to create a computing donation system where different entities can 
upload Cairo code. This code is then distributed with different arguments among the various clients connecting to the server 
to donate their computing power.


&nbsp;&nbsp;&nbsp;&nbsp;The donor will download the code of the project they want to contribute to, execute it in the Cairo VM, 
which will generate the execution trace. The output will be used by the client to generate the computational integrity proof, 
which will then be sent to the system server. The server will redirect this generated proof to the entity that uploaded the 
program for execution. The entity will verify the proof and handle the aggregation of accumulated data, meaning the processing 
relevant to the obtained data based on the problem to be solved.


&nbsp;&nbsp;&nbsp;&nbsp;The project will have a server where programs to be distributed are uploaded, a Command Line Interface 
(CLI) for organizations to interact with the server and process received proofs, and a CLI for those donating their computing 
power to choose which project to contribute to and start doing so. A small example of a project to be distributed, implemented 
in Cairo, will also be included. The system will be developed using the Rust programming language and will utilize the [virtual 
machine and prover](https://github.com/lambdaclass/lambdaworks) implemented by Lambdaclass.


&nbsp;&nbsp;&nbsp;&nbsp;As a computing donation system, consideration must be given to the computer's resources, adjusting 
usage based on system resource utilization to avoid consuming resources needed by the client and hindering the operating 
system's functionality. The option to solely generate an execution of a program for uploading to the program server, 
allowing another client to generate the program proof, will also be provided.


![System diagram](imgs/translated_system_diagram.jpg)

# Usage

This set of programs make use of docker, docker-compose and makefiles.

To install docker and docker-compose, you can use the following command in ubuntu:  
```
sudo snap install docker
```

In windows, you can either install [Docker desktop](https://www.docker.com/products/docker-desktop/), or use the 
previous command inside [wsl](https://learn.microsoft.com/en-us/windows/wsl/install).

If make is not installed in the computer, try executing the command 
```
sudo apt-get install build-essential
```

If for any reason the makefiles cannot be executed, then you can execute each command individually.


## Program distributor

The server that manages the uploaders' accounts, programs, inputs and proofs. It runs an http server
and it's makefile has the following command options:

- `make`: the same as make `run_all`
- `make run_all`: executes the docker compose file, setting up the database (which has a named volume), a phpmyadmin instance for database management, and the server itself.
- `make clear_build`: clears the server's containers and building named volume making it so that the program will have to be rebuilt from zero the next time it is executed.
- `make clear_docker`: clears the containers.
- `make clear_all`: runs `clear_docker` and `clear_build`.
- `make clear_db`: deletes the volume dedicated to the database data, clearing all the stored information.

The phpmyadmin mysql database manager can be accessed (if the `PHPMYADMIN_EXTERNAL_PORT` environment variable is not modified) by entering the following url in the browser:
[http://localhost:9000/](http://localhost:9000/).

The http server can be accessed (if the `SERVER_EXTERNAL_PORT` environment variable is not modified) by entering the following url in the browser:
[http://localhost:8080/](http://localhost:8080/).

### Configuration

#### dev.json

Inside the `src/config` folder you will find a file called qa.json, this contains a bare bones version of the config file that
must be put inside that folder under the `dev.json` name. The configuration file then must follow this format:  

```
{
    "database_url": String, // Database connection url
    "token": {
        "basic_token_secret": String, // Jwt token secret
        "basic_token_minutes_duration": u64, // Minutes between jwt token renovations
        "refresh_token_secret": String, // Refresh token secret
        "refresh_token_days_duration": u64 // Days until user is forced to log in again
    },
    "files_storage": {
        "connection_string": String, // S3 connection data in format "region{arguments_serarator}bucket_name{arguments_serarator}key_id{arguments_serarator}key_secret"
        "arguments_serarator": String // String that separates the arguments of connection_string
    }
}
```

In this implementation the server uses S3 for the program code and proofs storage, however, the user may implement another
interface for files storage if he desires. With the current implementation it is recommended to use the `:` character as
separator, resulting in a connection string of the format `region:bucket_name:key_id:key_secret`

#### .env

The `.env` contains variables assuming the database will be run with docker compose with the program distributor server, 
and it has the following format:

```

MYSQL_ROOT_PASSWORD=example
MYSQL_DATABASE=my_database
SERVER_EXTERNAL_PORT=8080
PHPMYADMIN_EXTERNAL_PORT=9000

DOCKERIZED_DATABASE_URL=mysql://root:example@db:3306/my_database

# This env variable is for diesel and should be used only to test the database schema generation
DATABASE_URL=mysql://root:example@127.0.0.1:3306/my_database
```

Please note that the program distributor server prioritizes the values of the environment variables over the
ones from the config file, so the variable `dockerized_database_url` from the docker compose file should never
be set if the user wants to connect to a remote database and set the value in dev.json, or set the value of `.env`'s
`DOCKERIZED_DATABASE_URL` with the remote database connection url.

### Endpoints

Responses formats:

- Ok with response body:

```
{
    "status": "success",
    "data": null | Object, // Data returned by the endpoint
}
```

- Error with response body:
```
{
    "status": "error",
    "error_code": String, // Error code
    "error_message": String, // Error description
}
```
Error codes:

1. "ACCOUNT_NOT_FOUND"  
Status_code: 404, NOT_FOUND

2. "PROGRAM_NOT_FOUND"  
Status_code: 404, NOT_FOUND

3. "PROGRAM_NAME_TAKEN"  
Status_code: 409, CONFLICT

4. "INPUT_GROUP_NOT_FOUND"  
Status_code: 404, NOT_FOUND

5. "BAD_BASE_64_ENCODING"  
Status_code: 422, UNPROCESSABLE_ENTITY

6. "WRONG_CREDENTIALS",  
Status_code: 403, FORBIDDEN

7. "USERNAME_ALREADY_EXISTS"  
Status_code: 409, CONFLICT

8. "REFRESH_TOKEN_NOT_FOUND"  
Status_code: 404, NOT_FOUND

9. "INVALID_TOKEN"  
Status_code: 403, FORBIDDEN

10. "INTERNAL_SERVER_ERROR"  
Status_code: 500, INTERNAL_SERVER_ERROR

#### Account

- `POST` `/account/register`

Endpoint for account registration

Json body format: 
```
{
    username: String, // Username chosen by the registering user
    password: String, // Password chosen by the registering user
    name: String, // Name of the organization, this value is public
    description: String, // Name of the organization, this value is public
}
```

Response: 
```
{
    "status": "success",
    "data": null
}
```
<br>

- `POST` `/account/login`

Endpoint for account login

Json body format: 
```
{
    username: String, // Username chosen by the user logging in
    password: String, // Password chosen by the user logging in
}
```

Response: 
```
{
    "status": "success",
    "data": {
        "basic_token": {
            "token_id": String, // Id of the jwt token
            "token": String, // Jwt token
        },
        "refresh_token": {
            "token_id": String, // id of the refresh token
            "token": String, // Refresh token
        }
    }
}
```
<br>

- `POST` `/account/refresh-token`

Endpoint for jwt token refreshment

Json body format: 
```
{
    pub refresh_token: String,
}
```

Response: 
```
{
    "status": "success",
    "data": {
        "token_id": String, // Id of the jwt token
        "token": String, // Jwt token
    }
}
```
<br>

- `GET` `/account/organizations`

Endpoint to get the registered accounts

Query parameters:
```
limit: i64, // Optional, Integer from 1 to 50
page: i64, // Optional, Integer from 1 onwards
name_filter: String, // Optional, String of the beginning of the organization's 
                     // name to filter the returned organizations
```
<br>

- `DELETE` `/account/refresh-token`

Endpoint to delete a refresh token, that is, logging out

Json body format: 
```
{
    token_id: String, // Id of the refresh token that is being deleted
}
```

Response:
```
{
    "status": "success",
    "data": {
        "organizations": [
            {
                "organization_id": String, // Organization's id
                "name": String, // Organization's public id
                "description": String, // Organization's description
            },
            ...
        ],
        "total_elements_amount": i64, // Positive integer or 0, the total amount of elements that would be
                                      // returned if there was no pagination
    }
}
```
<br>

#### Program

- `GET` `/program/all`  

Endpoint to get all the programs stored in the server with pagination

Query parameters:
```
limit: i64, // Optional, Integer from 1 to 50
page: i64, // Optional, Integer from 1 onwards
name_filter: String, // Optional, String of the beginning of the program's 
                     // name to filter the returned organizations
```

Response:
```
{
    "status": "success",
    "data": {
        "programs": [
            {
                "organization_id": String, // Program's owner organization id
                "program_id": String, // Program's id
                "name": String, // Program's name
                "description": String, // Program's description
                "input_lock_timeout": i64, // How many seconds will pass until the server considers 
                                           // the reservation of the program input as dropped
            },
            ...
        ],
        "total_elements_amount": i64, // Positive integer or 0, the total amount of elements that 
                                      // would be returned if there was no pagination
    }
}
```
<br>

- `GET` `/program/mine`

Returns a list with the programs uploaded by the logged in account.  

Headers:
```
token: String, // Jwt token
```

Query parameters:
```
limit: i64, // Optional, Integer from 1 to 50
page: i64, // Optional, Integer from 1 onwards
```

Response:
```
{
    "status": "success",
    "data": {
        "programs": [
            {
                "organization_id": String, // Program's owner organization id
                "program_id": String, // Program's id
                "name": String, // Program's name
                "description": String, // Program's description
                "input_lock_timeout": i64, // How many seconds will pass until the server considers the
                                           // reservation of the program input as dropped
            },
            ...
        ],
        "total_elements_amount": i64, // Positive integer or 0, the total amount of elements that would be
                                      // returned if there was no pagination
    }
}
```
<br>

- `GET` `/program/template`  

Returns a template for guest code implementation  

Response: `.tar` filestream

<br>

- `GET` `/program/inputs/{program_id}`  

Returns the file of one of the program's input group  

Response: `.csv` filestream

<br>

- `GET` `/program/inputs/all/{program_id}`  
Returns the input groups uploaded for the user's program.  

Headers:
```
token: String, // Jwt token
```

Query parameters:
```
limit: i64, // Optional, Integer from 1 to 50
page: i64, // Optional, Integer from 1 onwards
```

Response:
```
{
    "status": "success",
    "data": {
        program_input_groups: [
            {
                input_group_id: String, // Input group's id
                program_id: String, // Program's id
                name: String, // Program's name
                last_reserved: null | String, // Last time this input group was reserved
                proven_datetime: null | String, // Time when this input group was proven
            },
            ...
        ],
            total_elements_amount: i64, // Positive integer or 0, the total amount of elements that would be
                                        // returned if there was no pagination
        }
}
```
<br>


- `GET` `/program/program-and-inputs/{program_id}`  
Returns the program's code and the file of an input group  

Response: `.tar` filestream

<br>

- `GET` `/program/organization/{organization_id}`  
Returns the programs uploaded by a specific organization

Query parameters:
```
limit: i64, // Optional, Integer from 1 to 50
page: i64, // Optional, Integer from 1 onwards
```

Response:
```
{
    "status": "success",
    "data": {
        "programs": [
            {
                "organization_id": String, // Program's owner organization id
                "program_id": String, // Program's id
                "name": String, // Program's name
                "description": String, // Program's description
                "input_lock_timeout": i64, // How many seconds will pass until the server considers the
                                           // reservation of the program input as dropped
            },
            ...
        ],
        "total_elements_amount": i64, // Positive integer or 0, the total amount of elements that would be
                                      // returned if there was no pagination
    }
}
```

<br>

- `GET` `/program/proof/{program_id}/{input_group_id}`  
Returns the file of the program execution for a program input's proof uploaded by a donor.  

Headers:
```
token: String, // Jwt token
```

Response: `.bin` filestream

<br>

- `GET` `/program/proofs`  
Returns the uploaded programs of the user that have at least one proven input that needs to be verified

Headers:
```
token: String, // Jwt token
```

Query parameters:
```
limit: i64, // Optional, Integer from 1 to 50
page: i64, // Optional, Integer from 1 onwards
```

Response:
```
{
    "status": "success",
    "data": {
        "programs": [
            {
                "organization_id": String, // Program's owner organization id
                "program_id": String, // Program's id
                "name": String, // Program's name
                "description": String, // Program's description
                "input_lock_timeout": i64, // How many seconds will pass until the server considers the
                                           // reservation of the program input as dropped
            },
            ...
        ],
        "total_elements_amount": i64, // Positive integer or 0, the total amount of elements that would be
                                      // returned if there was no pagination
    }
}
```

- `GET` `/program/proofs/{program_id}`  
Returns the input groups uploaded for the user's program that have a proven execution.  

Headers:
```
token: String, // Jwt token
```

Query parameters:
```
limit: i64, // Optional, Integer from 1 to 50
page: i64, // Optional, Integer from 1 onwards
```

Response:
```
{
    "status": "success",
    "data": {
        program_input_groups: [
            {
                input_group_id: String, // Input group's id
                program_id: String, // Program's id
                name: String, // Program's name
                last_reserved: null | String, // Last time this input group was reserved
                proven_datetime: null | String, // Time when this input group was proven
            },
            ...
        ],
            total_elements_amount: i64, // Positive integer or 0, the total amount of elements that would be
                                        // returned if there was no pagination
    }
}
```
<br>

- `GET` `/program/{program_id}`  
Returns the program's code  

Response: `.tar` filestream

<br>

- `PATCH` `/program/proof/{program_id}/{input_group_id}`  
Deletes the uploaded proof and makes it so that another user can immediately make a reservation on it   

Headers:
```
token: String, // Jwt token
```

Response:
```
{
    "status": "success",
    "data": null,
}
```
<br>

- `DELETE` `/program/{program_id}`  
Deletes the user's uploaded program along with all the uploaded proofs and inputs   

Headers:
```
token: String, // Jwt token
```

Response:
```
{
    "status": "success",
    "data": null,
}
```
<br>

- `DELETE` `/program/proof/{program_id}/{input_group_id}`  
Deletes the user's input group uploaded proof, therefore confirming propper verification on the caller's part   

Headers:
```
token: String, // Jwt token
```

Response:
```
{
    "status": "success",
    "data": null,
}
```
<br>

- `DELETE` `/program/input/{program_id}/{input_group_id}`
Deletes the user's program input group along with the uploaded proof if there was any   

Headers:
```
token: String, // Jwt token
```

Response:
```
{
    "status": "success",
    "data": null,
}
```
<br>

- `POST` `/program/upload"`
Uploads a user program   

Headers:
```
token: String, // Jwt token
```

Form data:
```
file: File, // Tar file of the program's methods folder
data: Text, // Stringified json of the following struct:
{
    name: String, // Program's name
    description: String, // Program's description
    execution_timeout: i64, // Input group reservation timeout
}
```

Response:
```
{
    "status": "success",
    "data": {
        program_id: String, // Id of the uploaded program
    },
}
```
<br>

- `POST` `/program/proof`
Uploads a proof of execution of a program's input group  

Form data:
```
file: File, // Tar file of the program's methods folder
data: Text, // Stringified json of the following struct:
{
    organization_id: String, // Id of the organization that uploaded the proven program
    program_id: String, // Id of the program whose input group was proven
    input_group_id: String, // Id of the input group that was proven
}
```

Response:
```
{
    "status": "success",
    "data": null,
}
```
<br>

- `POST` `/program/inputs/{program_id}`
Lets a user upload an input group for the specified program   

Headers:
```
token: String, // Jwt token
```

Form data:
```
file: File, // Tar file of the program's methods folder
data: Text, // Stringified json of the following struct:
{
    name: String, // Input group's name
}
```

Response:
```
{
    "status": "success",
    "data": {
        input_group_id: String, // Id of the uploaded input group
    },
}
```
<br>

## Executor client



## Uploader client