# Contributing to halod

Contributions are restricted to the HALO team; therefore we will ignore all external pull requests or issues. Any part of this document might change in the future.

## Branch naming convention

All branch names should start with either of the following prefixes, depending on the increment they provide to the project:

| Prefix | Description |
| ------ | ----------- |
| feature | Adding a new feature to the project |
| fix | Fixing a bug |
| chore | Repository maintenance tasks |
| docs | Adding or modifying documentation |
| ci | CI/CD pipeline changes |
| test | Adding or modifying unit/integration tests |
| refactor | Restructuration of existing code |

Also, branch names should be in kebab case. For example: `feature/do-many-things`. Optionally, you can append the Jira ticket number to the branch name, like so: `feature/do-many-things-89`. Opening an issue before making a pull request is recommended but not mandatory.

## Pull requests

When the work on your branch is done, you can make a pull request to merge it into the main branch. Each pull request should follow the template given in `.github/pull_request_template.md`.
