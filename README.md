> [!CAUTION]
> Horizon is still in early development and is not meant to be used in any production environments.

![horizon-server-high-resolution-logo-transparent](/branding/horizon-server-high-resolution-logo-transparent.png)
<center>
<p style="text-align: center;" align="center">
    <img src="https://github.com/Stars-Beyond/Horizon-Community-Edition/actions/workflows/rust.yml/badge.svg" alt="Build">
    <img src="https://img.shields.io/github/repo-size/Stars-Beyond/Horizon-Community-Edition" alt="Size">
    <img src="https://img.shields.io/website?url=https%3A%2F%2Fstars-beyond.com%2F" alt="Website">
    <img src="https://img.shields.io/github/license/Stars-Beyond/Horizon-Community-Edition" alt="GitHub License">
    <img src="https://img.shields.io/github/discussions/Stars-Beyond/Horizon-Community-Edition" alt="GitHub Discussions">
    <img src="https://img.shields.io/github/sponsors/Stars-Beyond" alt="GitHub Sponsors">
    <img src="https://img.shields.io/github/forks/Stars-Beyond/Horizon-Community-Edition" alt="GitHub forks">
    <img src="https://img.shields.io/github/stars/Stars-Beyond/Horizon-Community-Edition" alt="GitHub Repo stars">
</p>
</center>


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

---
<h1 align="center" id='introduction'> 🚀 Introduction </h1>

Horizon is a custom game server software designed to facilitate seamless interaction between Unreal Engine 5 (UE5) and client applications through socket.io. It provides a scalable and customizable solution for hosting multiplayer games and managing real-time communication between players and a limitless number of game servers or "Hosts".

### Synchronized Game Server Architecture

Horizon utilizes a sophisticated Parent-Child socket synchronization mechanism to ensure seamless coordination between multiple Horizon instances, effectively creating a unified game server environment with minimal latency. This innovative approach enhances the overall performance and reliability of multiplayer gaming experiences by providing efficient and reliable synchronization across distributed server instances.

#### How it Works

##### Parent-Child Relationship

In the Horizon architecture, one instance acts as the Parent node, while others serve as Child nodes. The Parent node orchestrates and synchronizes actions across all Child nodes, ensuring that all game-related events and states are consistently and accurately propagated throughout the network of servers.

##### Socket Communication

Horizon employs socket.io for real-time communication between Parent and Child nodes. This allows for near-instantaneous data transmission, crucial for maintaining synchronization in fast-paced multiplayer games. Socket.io provides a robust and flexible framework for handling bi-directional event-based communication, enabling the seamless exchange of data between servers.

##### Data Exchange Protocol

The Parent node continuously sends updates to Child nodes regarding game state, player actions, and other relevant information. Conversely, Child nodes report back to the Parent node, ensuring bidirectional communication for accurate synchronization. This protocol ensures that all nodes are kept up-to-date with the latest game events, reducing the likelihood of discrepancies or inconsistencies in the game state.

##### Latency Optimization

To achieve near-zero latency, Horizon optimizes data transmission by minimizing overhead and prioritizing critical updates. This ensures that actions performed on one Child node propagate swiftly to all others, maintaining a cohesive game experience for all players. Techniques such as data compression, efficient serialization, and prioritization of critical packets are employed to reduce latency and enhance performance.

##### Dynamic Load Balancing

The system dynamically adjusts the load across Child nodes based on current server performance and player distribution. This load balancing mechanism helps prevent any single node from becoming a bottleneck, thereby enhancing the overall scalability and reliability of the server network.

##### Event Consistency

Each event generated within the game is timestamped and ordered to ensure consistency. The Parent node manages a global event timeline, allowing all Child nodes to process events in the correct sequence, maintaining a consistent game state across the entire network.

### Benefits

##### Scalability

The Parent-Child architecture allows Horizon to scale effortlessly, accommodating a growing player base without sacrificing performance. New Child nodes can be added to the network to handle increased load, and the Parent node can manage these additions seamlessly.

##### Fault Tolerance

In the event of node failure, the Parent node seamlessly redistributes responsibilities to remaining Child nodes, ensuring uninterrupted gameplay for players. This fault tolerance mechanism enhances the reliability and availability of the game server network, reducing the impact of hardware or software failures on the gaming experience.

##### Consistency

By synchronizing game state across all instances, Horizon guarantees a consistent experience for all players, regardless of their geographical location or server proximity. This consistency is crucial for maintaining fair and balanced gameplay, as all players interact with the same game state and events.

##### Improved User Experience

The low-latency and high-consistency nature of the Parent-Child synchronization model significantly enhances the user experience. Players benefit from faster response times, smoother gameplay, and reduced instances of lag or desynchronization.

##### Ease of Management

The centralized coordination provided by the Parent node simplifies the management of the game server network. Administrators can monitor and control the entire network from a single point, making it easier to deploy updates, manage resources, and troubleshoot issues.

##### Customizable Synchronization

Administrators can customize synchronization parameters, such as update frequency and data prioritization, to optimize performance based on specific game requirements. This flexibility allows Horizon to be tailored to a wide range of game types and player behaviors.

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

---
<h1 align="center" id='installation'> 🔧 Installation </h1>


### Prerequisites

Before installing Horizon, ensure that you have the following prerequisites:

- Docker installed on your system.
- Git for cloning the Horizon repository.
- Basic understanding of Docker and containerized applications.

### Installation Steps

1. Clone the Horizon repository from GitHub:

    ```bash
    git clone https://github.com/AstroVerse-Studios/Horizon.git
    ```

2. Navigate to the project directory:

    ```bash
    cd Horizon
    ```

3. Build and deploy the horizon service via docker-compose:

    ```bash
    docker-compose up --build
    ```

4. Follow the prompts to configure any necessary settings in the `server-config.json` file.

For more detailed instructions and troubleshooting tips, refer to the [Installation Guide](installation.md).


</br>

---
<h1 align="center" id='configuration'> ⚙️ Configuration </h1>


Horizon's configuration revolves around Docker and environment variables. Here's an overview of key configuration files:

- `compose.yaml`: Defines the Docker services, networks, and volumes for running Horizon.
- `Dockerfile`: Specifies the environment and dependencies for the Horizon server container.
- `start.sh`: Contains startup commands for launching the server.
- `server-config.json`: Contains Horizon server configurations.

To customize Horizon for your specific needs, modify these files according to your requirements. Refer to the [Configuration Guide](configuration.md) for detailed instructions and best practices.


</br>

---
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

---
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

---
<h1 align="center" id='additional-resources'> 📚 Additional Resources </h1>


### Community Support

- Join our Discord server or community forums for support and collaboration.
- Follow us on social media for updates and announcements.

### Documentation

- Explore the official Horizon documentation for in-depth guides and tutorials.
- Check out our GitHub repository for code samples and examples.

For more resources and helpful links, visit the [Additional Resources section](resources.md).


</br>

---
<h1 align="center" id='troubleshooting'> 🐞 Troubleshooting </h1>


### Common Issues

- **Connection Errors**: Ensure that the server is running and accessible from client applications.
- **Dependency Problems**: Check Docker logs for any issues during container initialization.
- **Performance Bottlenecks**: Monitor server performance and optimize resource usage if necessary.

For troubleshooting tips and solutions to common problems, consult the [Troubleshooting Guide](troubleshooting.md).


</br>

---
<h1 align="center" id='stargazers'> 🌟 Stargazers </h1>


<a href="https://github.com/Stars-Beyond/Horizon-Community-Edition/stargazers/">
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="http://reporoster.com/stars/Stars-Beyond/Horizon-Community-Edition">
    <img alt="stargazer-widget" src="http://reporoster.com/stars/dark/Stars-Beyond/Horizon-Community-Edition">
  </picture>
</a>

