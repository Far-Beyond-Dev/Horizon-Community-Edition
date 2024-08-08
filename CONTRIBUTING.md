# Contributing to Horizon

Thank you for your interest in contributing to Horizon! Your contributions are valuable to us. This document provides guidelines to help you get started, maintain consistency, and make the development process smooth for everyone involved.

## Table of Contents

- [Contributing to Horizon](#contributing-to-horizon)
  - [Table of Contents](#table-of-contents)
  - [Code of Conduct](#code-of-conduct)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Setting Up Your Development Environment](#setting-up-your-development-environment)
  - [Making Changes](#making-changes)
    - [Branching](#branching)
    - [Coding Standards](#coding-standards)
    - [Testing](#testing)
  - [Submitting a Pull Request](#submitting-a-pull-request)
  - [Code Reviews](#code-reviews)
  - [Additional Resources](#additional-resources)

## Code of Conduct

We are committed to fostering a welcoming and inclusive community. Please read and adhere to our [Code of Conduct](CODE_OF_CONDUCT.md) to ensure a positive experience for all contributors.

## Getting Started

### Prerequisites

Before you start contributing, ensure you have the following installed:

- [Docker](https://www.docker.com/get-started)
- [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
- Basic knowledge of Docker and containerized applications.

### Setting Up Your Development Environment

1. **Clone the Repository**

   ```bash
   git clone https://github.com/AstroVerse-Studios/Horizon.git
   cd Horizon
   ```

2. **Install Dependencies**

   Navigate to the project directory and install dependencies using `npm` or `yarn`:

   ```bash
   ./setup.sh
   ```

3. **Build the Project**

   Use Docker to build the Horizon service:

   ```bash
   docker-compose up --build
   ```

4. **Configure Environment Variables**

   Copy the sample configuration file and update the settings as necessary:

   ```bash
   cp .env.sample .env
   # Edit .env with your configurations
   ```

## Making Changes

### Branching

1. **Create a Branch**

   Always create a new branch for your changes. Use a descriptive name for your branch, e.g., `feature/add-new-endpoint` or `bugfix/resolve-issue`.

   ```bash
   git checkout -b feature/add-new-endpoint
   ```

2. **Commit Your Changes**

   Write clear, concise commit messages. Follow the conventional commit format:

   ```bash
   git commit -m "feat: add new endpoint for user authentication"
   ```

### Coding Standards

Follow the coding standards used in the project. Ensure code consistency by adhering to:

- **JavaScript/TypeScript:** Use `ESLint` with the project’s configuration.
- **Python:** Follow PEP 8 guidelines.

### Testing

1. **Run Tests**

   Ensure all tests pass before submitting your pull request:

   ```bash
   ./test.sh
   ```

2. **Write Tests**

   Add tests for new features or bug fixes. Use the existing testing framework and structure:

   ```javascript
   // Example test case
   test('should return the correct result', () => {
     expect(myFunction()).toBe(expectedValue);
   });
   ```

## Submitting a Pull Request

1. **Push Your Branch**

   Push your changes to your forked repository:

   ```bash
   git push origin feature/add-new-endpoint
   ```

2. **Create a Pull Request**

   Navigate to the GitHub repository and create a new pull request. Provide a clear description of your changes, reference any relevant issues, and explain the rationale behind your modifications.

3. **Request Review**

   Tag reviewers and ensure you follow any discussion threads. Be open to feedback and willing to make changes based on review comments.

## Code Reviews

- **Respond Promptly:** Address feedback and comments from reviewers promptly.
- **Be Constructive:** Provide and accept constructive feedback.
- **Maintain Respect:** Uphold a respectful and collaborative tone in all discussions.

## Additional Resources

- **Documentation:** [Horizon Documentation](https://github.com/Stars-Beyond/Horizon-Community-Edition/wiki)
- **Community:** Join our Discord server or community forums.
- **Issue Tracker:** Report bugs and request features on [GitHub Issues](https://github.com/Stars-Beyond/Horizon-Community-Edition/issues/new/choose).


---

Thank you for your contributions! Together, we can make Horizon an amazing game server solution.
