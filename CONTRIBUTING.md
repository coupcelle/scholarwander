# Contributing to ScholarWander

Here are some things you might need to get started contributing to this project

## Configs from your universities Eduroam setup program

There is a configuration file that SecureW2 uses to know how to set up your connection. THis is needed if you want to be able to run ScholarWander.

To get this config information, navigate to your universities portal for setting up wifi/eduroam (typically `wifi.<org>.edu` or `<org>.edu/wifi`). Select "Linux" as the download option and continue to the download. You should end up with a file ending in `.run`.

For development purposes, you may also want to find some configs from some other universities to ensure that the code is portable regardless of the config. The download portals for other universities can be found using a google search such as `"SecureW2" site:.edu`. Everything should be accessible without authentication. Please do not attempt to bypass any login screens or authentication mechanisms in order to get configuration information.

### Extracting the config from the `.run`
To extract the config file (`.cloudconfig`) from the installer, use the command `sed -e '0,/^#ARCHIVE#$/d' /path/to/file.run | tar zxf - SecureW2.cloudconfig`, this should spit out a `SecureW2.cloudconfig` file in your current directory.



## Diagrams

If you are interested in participating in development, you will likely also want to download [StarUML](https://staruml.io/) - this allows you to open some of the diagrams in this repository to understand how things are meant to work. 


## Team Communication

If you discovered this project through your Linux Users Group, reach out to the person who referred you for information about how to join the team chats. We may create a more public chat option soon to further improve collaboration.

