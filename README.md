> [!WARNING]
> Horizon is still in early development and is not meant to be used in any production environments.

> [!NOTE]
> Our installation methods have recently changed, please review them if you are experiencing difficulty with the installation process

<div align="center">

![horizon-server-high-resolution-logo-transparent](/branding/horizon-server-high-resolution-logo-transparent.png)

[![Discord](https://img.shields.io/discord/538264063956877312?label=discord)](https://discord.gg/NM4awJWGWu)
[![GitHub Actions](https://img.shields.io/github/actions/workflow/status/Far-Beyond-Dev/Horizon-Community-Edition/main.yml)](https://github.com/Far-Beyond-Dev/Horizon-Community-Edition/actions)
[![License](https://img.shields.io/badge/License-Apache--2.0-green.svg)](https://github.com/Far-Beyond-Dev/Horizon-Community-Edition/blob/main/LICENSE)
![Visits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2FFar-Beyond-Dev%2FHorizon-Community-Edition&count_bg=%2379C83D&title_bg=%23555555&icon_color=%23E7E7E7&title=Visits)
![Repo Size](https://img.shields.io/github/repo-size/Stars-Beyond/Horizon-Community-Edition)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/Far-Beyond-Dev)](https://github.com/sponsors/Far-Beyond-Dev)
[![Website](https://img.shields.io/website?url=https%3A%2F%2Fstars-beyond.com%2F)](https://horizon.farbeyond.dev) <!-- can i make them flat? -->

An easily scalable game server implemented in Rust, and compatible with many popular game engines
</div>

## Table Of Contents


- [📝 Table Of Contents](#table-of-contents)
- [🚀 Introduction](#introduction)
  - [Synchronized Game Server Architecture](#synchronized-game-server-architecture)
    - [How it Works](#how-it-works)
    - [Benefits](#benefits)
    - [Implementation Details](#implementation-details)
    - [Event Propagation and Multicasting](#event-propagation-and-multicasting)
    - [Coordinate Management and Region Mapping](#coordinate-management-and-region-mapping)
- [🔧 Installation](#installation)
  - [Prerequisites](#prerequisites)
  - [Installation Steps](#installation-steps)
- [⚙️ Configuration](#configuration)
- [📈 Usage](#usage)
  - [Starting the Server](#starting-the-server)
  - [Managing the Server manually](#managing-the-server-manually)
- [💻 Development](#development)
  - [Contributors](#contributors)
  - [Project Structure](#project-structure)
  - [Contribution Guidelines](#contribution-guidelines)
- [📚 Additional Resources](#additional-resources)
  - [Community Support](#community-support)
  - [Documentation](#documentation)
- [🐞 Troubleshooting](#troubleshooting)
  - [Common Issues](#common-issues)
- [🌟 Stargazers](#stargazers)


</br>

<h1 align="center" id='introduction'> 🚀 Introduction </h1>

Horizon is a custom game server software designed to facilitate seamless interaction between Unreal Engine 5 (UE5) and client applications through [socket.io](https://socket.io). It provides a scalable and customizable solution for hosting multiplayer games and managing real-time communication between players and a limitless number of game servers or "Hosts".

## Synchronized Game Server Architecture

Horizon offers two distinct architectural models for game server synchronization:

---

### 1. Peer-to-Peer Model (Community Edition)

In the Community Edition, Horizon utilizes a peer-to-peer model for synchronizing multiple game server instances. This approach allows for efficient communication and coordination between servers without the need for a central authority.

#### How it Works

- Each Horizon server instance operates as an equal peer in the network.
- Servers communicate directly with each other to share game state updates, player actions, and other relevant information.
- The peer-to-peer model enables horizontal scalability, allowing new server instances to be added seamlessly to the network.

#### Benefits

- Decentralized architecture, reducing single points of failure.
- Lower operational complexity, ideal for smaller deployments or community-driven projects.
- Efficient resource utilization across all participating servers.

### 2. Parent-Child-Master Architecture (Enterprise Edition)

For larger-scale deployments and enterprise use cases, Horizon offers an advanced Parent-Child-Master architecture. This model provides enhanced control, scalability, and management capabilities.

#### How it Works

- **Master Node**: Oversees the entire network, managing global state and coordination.
- **Parent Nodes**: Act as regional coordinators, managing a subset of Child nodes.
- **Child Nodes**: Handle individual game instances or regions, reporting to their Parent node.

This hierarchical structure allows for more sophisticated load balancing, fault tolerance, and centralized management as well as limitless scalability.

![diagram](/diagrams/96bdd2a1-e17a-44a2-b07b-04eacbdec4eb.png) <!-- gimme a better name! --> <!-- make the images local -->
<p align="center">Server image was created by Freepik</p>

#### Benefits

- Highly scalable architecture suitable for massive multiplayer environments.
- Advanced load balancing and resource allocation capabilities.
- Centralized monitoring and management through the Master node.
- Enhanced fault tolerance and redundancy options.

---

### Choosing the Right Architecture

- The Peer-to-Peer model (Community Edition) is ideal for smaller projects, community servers, or deployments that prioritize simplicity and decentralization.
- The Parent-Child-Master architecture (Enterprise Edition) is designed for large-scale commercial games, MMOs, or any project requiring advanced management and scalability features.

Both architectures leverage Horizon's core strengths in real-time synchronization and efficient data propagation, ensuring a consistent and responsive gaming experience regardless of the chosen model.

---

### Implementation Details

##### Configuration

Administrators can fine-tune synchronization parameters via the `server-config.json` file, adjusting settings such as synchronization frequency and data prioritization to suit specific requirements.

##### Monitoring

Horizon provides built-in monitoring tools to track synchronization performance, allowing administrators to identify and address any potential bottlenecks or issues promptly.

### Event Propagation and Multicasting

Horizon implements a robust event propagation mechanism to facilitate communication between servers based on spatial proximity and event origin.

##### Multicast System

Events are multicast from the Parent node to Child nodes based on their geographical proximity and relevance to the event origin. This ensures that only necessary servers receive and process the events, optimizing network bandwidth and computational resources.

##### Propagation Distance

Each event carries a propagation distance parameter, allowing servers to determine whether they should propagate the event further or handle it locally based on their position relative to the event origin.

### Coordinate Management and Region Mapping

##### Spatial Coordinates

Horizon uses a 64-bit floating-point coordinate system to manage server positions within a simulated universe. Each server instance covers a cubic light year, and coordinates are stored relativistically to avoid overflow issues.

##### Region Mapping

Servers are organized into a grid-based region map, where each region corresponds to a specific set of spatial coordinates. This mapping enables efficient routing of events between servers, as servers can quickly determine which neighboring servers should receive specific events based on their region coordinates.

</br>

<h1 align="center" id='installation'> 🔧 Installation </h1>


### Prerequisites

Before installing Horizon, ensure that you have the following prerequisites:

- Docker installed on your system.
- Git for cloning the Horizon repository.
- Basic understanding of Docker and containerized applications.

### Installation Steps

1. Clone the Horizon repository from GitHub:

    ```bash
    git clone https://github.com/Far-Beyond-Dev/Horizon-Community-Edition.git
    ```
    ![image](https://github.com/user-attachments/assets/e48ce2eb-28f6-44aa-800c-7c0b7c79eb94)


2. Navigate to the project directory:

    ```bash
    cd Horizon-Community-Edition/
    ```
    ![image](https://github.com/user-attachments/assets/4a186d60-3da6-44b1-8e4c-c21c23360dd7)


3. Enter WSL or open the project in a [VSCode Devcontainer](https://code.visualstudio.com/docs/devcontainers/containers#_open-an-existing-workspace-in-a-container)

    ```bash
    wsl
    ```

4. Run the following command to install the dependencies, this script only works with Ubuntu, Arch & Alpine Linux

    ```bash
    ./installer-linux.sh
    ```
    ![image](https://github.com/user-attachments/assets/74a91770-25f8-43fc-97f3-202136b46250)

6. Use cargo to compile and run Horizon

    ```bash
    cargo run
    ```
    ![image](https://github.com/user-attachments/assets/aacdec93-88ff-4d51-a206-96b9d6a316eb)


7. Follow the prompts to configure any necessary settings in the `server-config.json` file.

For more detailed instructions and troubleshooting tips, refer to the [Installation Guide](installation.md).


</br>

<h1 align="center" id='configuration'> ⚙️ Configuration </h1>


Horizon's configuration revolves around Docker and environment variables. Here's an overview of key configuration files:

- `compose.yaml`: Defines the Docker services, networks, and volumes for running Horizon.
- `Dockerfile`: Specifies the environment and dependencies for the Horizon server container.
- `start.sh`: Contains startup commands for launching the server.
- `server-config.json`: Contains Horizon server configurations.

To customize Horizon for your specific needs, modify these files according to your requirements. Refer to the [Configuration Guide](configuration.md) for detailed instructions and best practices.


</br>

<h1 align="center" id='usage'> 📈 Usage </h1>


### Starting the Server

To start the Horizon server, execute the following command (This assumes you have already [Built the Horizon server](#installation-steps)):

```bash
./start.sh
```

This script initializes the Docker containers and launches the server. Once started, you can connect to the server using socket.io clients or integrate it with your Unreal Engine 5 project.

### Managing the Server manually

You can manage your server directly via docker-compose:

- Use `docker-compose` commands to manage the server lifecycle (e.g., `docker-compose up`, `docker-compose down`).
- Monitor server logs for debugging and performance analysis.

For more usage instructions and advanced features, see the [Usage Guide](usage.md).


</br>

<h1 align="center" id='development'> 💻 Development </h1>


### Contributors

Contributors help shape the future of Horizon Server. To start contributing you have to [fork this repository](https://github.com/Stars-Beyond/Horizon-Community-Edition/fork) and [open a pull request](https://github.com/Stars-Beyond/Horizon-Community-Edition/compare).

<a href="https://github.com/Stars-Beyond/Horizon-Community-Edition/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Stars-Beyond/Horizon-Community-Edition"/>
</a>

### Project Structure

The Horizon project directory consists of several key components:

- `src/`: Contains source code for the Horizon server.
- `horizon-physics-engine/`: Additional modules or plugins for extended functionality.
- `BuntDB/`: Database-related files and configurations.
- Other configuration files and scripts for Docker and environment setup.

### Contribution Guidelines

- Follow the project's coding standards and conventions.
- Submit pull requests for proposed changes or enhancements.
- Collaborate with the community on GitHub issues and discussions.

For detailed development instructions and guidelines, refer to the [Development Guide](development.md).


</br>

<h1 align="center" id='additional-resources'> 📚 Additional Resources </h1>


### Community Support

- Join our Discord server or community forums for support and collaboration.
- Follow us on social media for updates and announcements.

### Documentation

- Explore the official Horizon documentation for in-depth guides and tutorials.
- Check out our GitHub repository for code samples and examples.

For more resources and helpful links, visit the [Additional Resources section](resources.md).


</br>

<h1 align="center" id='troubleshooting'> 🐞 Troubleshooting </h1>


### Common Issues

- **Connection Errors**: Ensure that the server is running and accessible from client applications.
- **Dependency Problems**: Check Docker logs for any issues during container initialization.
- **Performance Bottlenecks**: Monitor server performance and optimize resource usage if necessary.

For troubleshooting tips and solutions to common problems, consult the [Troubleshooting Guide](troubleshooting.md).


</br>

<h1 align="center" id='stargazers'> 🌟 Stargazers </h1>

<a href="https://github.com/Stars-Beyond/Horizon-Community-Edition/stargazers/">
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="http://reporoster.com/stars/Stars-Beyond/Horizon-Community-Edition">
    <img alt="stargazer-widget" src="https://reporoster.com/stars/dark/Far-Beyond-Dev/Horizon-Community-Edition">
  </picture>
</a>
