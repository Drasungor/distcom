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

If for any the makefiles cannot be executed, then you can execute each command individually.


## Program distributor

## Executor client

## Uploader client