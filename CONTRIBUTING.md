# Contributing to Horizon

Thank you for your interest in contributing to Horizon! We value the contributions of our community members and are excited to have you on board. This document provides detailed guidelines to help you get started, maintain consistency, and make the development process smooth for everyone involved.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
   - [Prerequisites](#prerequisites)
   - [Setting Up Your Development Environment](#setting-up-your-development-environment)
3. [Making Contributions](#making-contributions)
   - [Finding Issues to Work On](#finding-issues-to-work-on)
   - [Creating a Pull Request](#creating-a-pull-request)
4. [Coding Standards](#coding-standards)
5. [Testing](#testing)
6. [Documentation](#documentation)

## Code of Conduct

We are committed to fostering an inclusive and welcoming community. Please read and adhere to our [Code of Conduct](CODE_OF_CONDUCT.md) in all interactions.

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed on your system:

- Git
- Docker
- Visual Studio Code (VSCode)

### Setting Up Your Development Environment

1. **Clone the repository:**
   ```
   git clone https://github.com/your-organization/horizon.git
   cd horizon
   ```

2. **Clone all submodules:**
   ```
   git submodule update --init --recursive
   ```

3. **Install Docker:**
   If you haven't already, download and install Docker from [docker.com](https://www.docker.com/). Follow the installation instructions for your operating system.

4. **Start Docker engine:**
   Ensure the Docker daemon is running on your system.

5. **Install the Dev Containers extension in VSCode:**
   - Open VSCode
   - Go to the Extensions view (Ctrl+Shift+X or Cmd+Shift+X)
   - Search for "Dev Containers"
   - Click Install

6. **Open the project in VSCode:**
   ```
   code .
   ```

7. **Open the Command Palette in VSCode:**
   - Windows/Linux: Ctrl+Shift+P
   - Mac: Cmd+Shift+P

8. **Set up the Dev Container:**
   - Type "Open Folder in Container" in the Command Palette
   - Select this option and press Enter
   - VSCode will build and start the container (this may take a few minutes the first time)

9. **Run the installers script:**
   Once inside the Dev Container, open a terminal in VSCode and run:
   ```
   ./installers-deb.sh
   ```

You're now set up and ready to contribute to Horizon!

## Making Contributions

### Finding Issues to Work On

- Check our [Issues](https://github.com/your-organization/horizon/issues) page for open tasks.
- Look for issues tagged with `good first issue` or `help wanted`.
- If you have an idea for a new feature, please open an issue to discuss it before starting work.

### Creating a Pull Request

1. Create a new branch for your work:
   ```
   git checkout -b feature/your-feature-name
   ```

2. Make your changes and commit them with a clear, descriptive commit message.

3. Push your branch to your fork:
   ```
   git push origin feature/your-feature-name
   ```

4. Go to the Horizon repository on GitHub and create a new Pull Request.

5. Fill out the Pull Request template with all relevant information.

6. Wait for review. We aim to review PRs within a week.

## Coding Standards

- Follow the existing code style in the project.
- Use meaningful variable and function names.
- Comment your code where necessary, especially for complex logic.
- Write clean, readable, and maintainable code.

## Testing

- Ensure all GitHub actions checks pass before pubmitting your PR.

## Documentation

- Update relevant documentation when making changes.
- If you're adding new features, include appropriate documentation.
- Use clear and concise language in your documentation.


Thank you for contributing to Horizon! Your efforts help make our project better for everyone.
