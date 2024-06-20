> [!CAUTION]
> Horizon is still in early development and is not meant to be used in any production environments.

![horizon-server-high-resolution-logo-transparent](/branding/horizon-server-high-resolution-logo-transparent.png)

## Table Of Contents

- [Table Of Contents](#table-of-contents)
- [1. Introduction](#1-introduction)
  - [Synchronized Game Server Architecture](#synchronized-game-server-architecture)
    - [Horizon Parent-Child Socket Sync](#horizon-parent-child-socket-sync)
      - [How it Works](#how-it-works)
      - [Benefits](#benefits)
      - [Implementation Details](#implementation-details)
    - [Server-to-Server Communication](#server-to-server-communication)
      - [Event Propagation and Multicasting](#event-propagation-and-multicasting)
      - [Coordinate Management and Region Mapping](#coordinate-management-and-region-mapping)
  - [Conclusion](#conclusion)
- [2. Installation](#2-installation)
    - [Prerequisites](#prerequisites)
    - [Installation Steps](#installation-steps)
- [3. Configuration](#3-configuration)
- [4. Usage](#4-usage)
  - [Starting the Server](#starting-the-server)
  - [Managing the Server manually](#managing-the-server-manually)
- [5. Development](#5-development)
  - [Project Structure](#project-structure)
  - [Contribution Guidelines](#contribution-guidelines)
- [6. Additional Resources](#6-additional-resources)
  - [Community Support](#community-support)
  - [Documentation](#documentation)
- [7. Troubleshooting](#7-troubleshooting)
  - [Common Issues](#common-issues)

---

## 1. Introduction

Horizon is a custom game server software designed to facilitate seamless interaction between Unreal Engine 5 (UE5) and client applications through socket.io. It provides a scalable and customizable solution for hosting multiplayer games and managing real-time communication between players and a limitless number of game servers or "Hosts".

### Synchronized Game Server Architecture

#### Horizon Parent-Child Socket Sync

Horizon utilizes a sophisticated Parent-Child socket synchronization mechanism to ensure seamless coordination between multiple Horizon instances, effectively creating a unified game server environment with minimal latency.

##### How it Works

- **Parent-Child Relationship**: In the Horizon architecture, one instance acts as the Parent node, while others serve as Child nodes. The Parent node orchestrates and synchronizes actions across all Child nodes.

- **Socket Communication**: Horizon employs socket.io for real-time communication between Parent and Child nodes. This allows for near-instantaneous data transmission, crucial for maintaining synchronization in fast-paced multiplayer games.

- **Data Exchange Protocol**: The Parent node continuously sends updates to Child nodes regarding game state, player actions, and other relevant information. Conversely, Child nodes report back to the Parent node, ensuring bidirectional communication for accurate synchronization.

- **Latency Optimization**: To achieve near-zero latency, Horizon optimizes data transmission by minimizing overhead and prioritizing critical updates. This ensures that actions performed on one Child node propagate swiftly to all others, maintaining a cohesive game experience for all players.

##### Benefits

- **Scalability**: The Parent-Child architecture allows Horizon to scale effortlessly, accommodating a growing player base without sacrificing performance.

- **Fault Tolerance**: In the event of node failure, the Parent node seamlessly redistributes responsibilities to remaining Child nodes, ensuring uninterrupted gameplay for players.

- **Consistency**: By synchronizing game state across all instances, Horizon guarantees a consistent experience for all players, regardless of their geographical location or server proximity.

##### Implementation Details

- **Configuration**: Administrators can fine-tune synchronization parameters via the `server-config.json` file, adjusting settings such as synchronization frequency and data prioritization to suit specific requirements.

- **Monitoring**: Horizon provides built-in monitoring tools to track synchronization performance, allowing administrators to identify and address any potential bottlenecks or issues promptly.

#### Server-to-Server Communication

##### Event Propagation and Multicasting

Horizon implements a robust event propagation mechanism to facilitate communication between servers based on spatial proximity and event origin.

- **Multicast System**: Events are multicast from the Parent node to Child nodes based on their geographical proximity and relevance to the event origin. This ensures that only necessary servers receive and process the events, optimizing network bandwidth and computational resources.

- **Propagation Distance**: Each event carries a propagation distance parameter, allowing servers to determine whether they should propagate the event further or handle it locally based on their position relative to the event origin.

##### Coordinate Management and Region Mapping

- **Spatial Coordinates**: Horizon uses a 64-bit floating-point coordinate system to manage server positions within a simulated universe. Each server instance covers a cubic light year, and coordinates are stored relative to avoid overflow issues.

- **Region Mapping**: Servers are organized into a grid-based region map, where each region corresponds to a specific set of spatial coordinates. This mapping enables efficient routing of events between servers, as servers can quickly determine which neighboring servers should receive specific events based on their region coordinates.

### Conclusion

The Horizon Parent-Child socket synchronization mechanism and server-to-server communication protocols provide a robust foundation for managing distributed multiplayer game environments. By leveraging efficient event propagation, coordinate management, and real-time synchronization techniques, Horizon ensures a seamless and immersive gaming experience for all players.

---

## 2. Installation

#### Prerequisites
Before installing Horizon, ensure that you have the following prerequisites:

- Docker installed on your system.
- Git for cloning the Horizon repository.
- Basic understanding of Docker and containerized applications.

#### Installation Steps

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

---

## 3. Configuration

Horizon's configuration revolves around Docker and environment variables. Here's an overview of key configuration files:

- `compose.yaml`: Defines the Docker services, networks, and volumes for running Horizon.
- `Dockerfile`: Specifies the environment and dependencies for the Horizon server container.
- `start.sh`: Contains startup commands for launching the server.
- `server-config.json`: Contains Horizon server configurations.

To customize Horizon for your specific needs, modify these files according to your requirements. Refer to the [Configuration Guide](configuration.md) for detailed instructions and best practices.

---

## 4. Usage

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

---

## 5. Development

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

---

## 6. Additional Resources

### Community Support

- Join our Discord server or community forums for support and collaboration.
- Follow us on social media for updates and announcements.

### Documentation

- Explore the official Horizon documentation for in-depth guides and tutorials.
- Check out our GitHub repository for code samples and examples.

For more resources and helpful links, visit the [Additional Resources section](resources.md).

---

## 7. Troubleshooting

### Common Issues

- **Connection Errors**: Ensure that the server is running and accessible from client applications.
- **Dependency Problems**: Check Docker logs for any issues during container initialization.
- **Performance Bottlenecks**: Monitor server performance and optimize resource usage if necessary.

For troubleshooting tips and solutions to common problems, consult the [Troubleshooting Guide](troubleshooting.md).
