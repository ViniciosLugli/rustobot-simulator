# Rustobot Simulator
Open source simulator that integrates Automation and Information Technologies, allowing to control the intelligent robot Dobot Magician Lite in a simulated way using Rust and Godot engine.

## Goals of project
Backend:
- [ ] Develop a backend capable of storing information such as robot displacement, position, and sending real-time information to a simulation environment.
	- [ ] Determine the database structure and choose a database management system (DBMS).
	- [ ] Implement a RESTful API to handle incoming and outgoing data from clients.
	- [ ] Implement middleware to handle data processing and communication between the API and the database.

Frontend:
- [ ] Develop a frontend application that allows users to interact with the system.
	- [ ] Design the user interface (UI) and user experience (UX) of the application.
	- [ ] Implement a feature that allows users to request robot movements in joint or global coordinates.
	- [ ] Implement a feature that allows users to view the current position of the robot in the UI.
	- [ ] Implement a feature that stores each robot movement requested by the user.

Simulator:
- [ ] Implement a system for simulating the robotic arm's behavior using a 3D representation of its kinematic chain.
	- [ ] Choose a game engine for implementing the simulation, such as Godot ( probably it will be used ).
	- [ ] Implement a mechanism for requesting the API to update the robot's target position.
	- [ ] Implement a mechanism for receiving the robot's current position from the API and updating it in the simulation.

## Setup environment for development

### Clone the repository

```bash
git clone git@github.com:ViniciosLugli/rustobot-simulator.git
```

### Set the environment variables (.env)
Is necessary to create a file called `.env` in the folder of each workspace, and set the environment variables from template:
#### Server

```bash
# Database ( default of postgres docker image )
DATABASE_HOST = "localhost"
DATABASE_PORT = 5432
DATABASE_USER = "postgres"
DATABASE_PASSWORD = "postgres"
DATABASE_NAME = "rustobot"

DATABASE_URL = postgresql://${DATABASE_USER}:${DATABASE_PASSWORD}@${DATABASE_HOST}:${DATABASE_PORT}/${DATABASE_NAME}

# API
WORKERS = 2

# App
RUST_LOG = "debug"
```
Final path of the file: `server/.env`

_💊 Yes, currently is only needed to set the environment variables for the server workspace_

#### Interface
The interface workspace has one configuration file, the `Trunk.toml`, that is used to configure the build of the project, the file is located in the root folder of the project, and the default configuration is:
```toml
[build]
dist = "dist/"
target = "index.html"

[serve]
address = "0.0.0.0"
port = 8080
```
Is not necessary to put the environment variables in the file, because the variables are already in [interface/Trunk.toml](./interface/Trunk.toml) path, but if you want to change the default configuration, you can do it in the file.


### Install docker and docker-compose

You can find the installation instructions for your operating system here:

1.  docker: https://docs.docker.com/get-docker/
2.  docker-compose: https://docs.docker.com/compose/install

_⚠️ Warning: Install docker first then docker-compose_

#### Verify installations

```bash
docker --version
```

```bash
docker-compose --version
```

_🍀 Tip: Is recommended use the docker-desktop app to manage and monitor the containers if you are not familiar with docker_

### Running the container
In the root folder of the project, run the following command to run all the containers:
```bash
COMPOSE_PROFILES=development docker-compose up
```
The COMPOSE_PROFILES environment variable is used to define the profiles that will be used to run the container. All the profiles are defined in the [docker-compose](./docker-compose.yml) file:
-  `development`: runs the container in development mode ( with local database )
-  `production`: runs the container in production mode ( without local database)
- `test`: runs the container for test workspaces

_🍀 Tips: If you want to run the container in the background, use the `-d` flag, and if you need to rebuild the container, use the `--build` flag after the command, see more in the [docker-compose documentation](https://docs.docker.com/compose/reference/up/) documentation_


#### Last tested docker version:

-   Docker version 23.0.1
-   Docker Compose version v2.15.1 with yml version 3.8


#### For modifications in the code

the [docker-compose](./docker-compose.yml) is configured to run the each workspace container, currently the workspaces are:
- [interface](./interface/Dockerfile)
- [server](./server/Dockerfile)

_🤘 Reminder: If you need modify the Dockerfile of the workspaces, you need to rebuild the container with the `--build` flag, to take effect the changes, see more in the [docker-compose documentation](https://docs.docker.com/compose/reference/up/) documentation_