# Contributing to ScholarWander

Here are some things you might need to get started contributing to this project

## Configs from your universities Eduroam setup program

See the README for info on how to get yourself a config and extract it

For development purposes, you may also want to find some configs from some other universities to ensure that the code is portable regardless of the config. The download portals for other universities can be found using a google search such as `"SecureW2" site:.edu`. Everything should be accessible without authentication. Please do not attempt to bypass any login screens or authentication mechanisms in order to get configuration information.


## Diagrams

If you are interested in participating in development, you will likely also want to download [StarUML](https://staruml.io/) - this allows you to open some of the diagrams in this repository to understand how things are meant to work. 


## Team Communication

If you discovered this project through your Linux Users Group, reach out to the person who referred you for information about how to join the team chats. We may create a more public chat option soon to further improve collaboration.


## Task tracker
Talk to the team about this if you want tasks


## Workflow and Pull Request standards

The workflow is essentially Github Flow/Fork and pull request

Wherever possible, especially when making a pull request, make sure:
- Any new code is unit tested as much as reasonably possible
- no identifying information is present in code or tests (such as real identifiers from real config data)
- do not commit any secrets or anything that requires authentication in order to get.
- try to keep your code formatted, well documented, and idiomatic to common rust conventions. You cna use `clippy` if you want to check your code

