<h1 align="center">
    Charon
</h1>

<h4 align="center">
    Command line interface to create new workspaces based on templates
</h4>

## Description

Charon enables you to effectively create new projects based on predefined templates, by utilizing a simple and minimal usage syntax. Templates can be shared and managed accross teams through git. Templates are automaticly pulled and updated on all clients.

## Contents

- [Description](#description)
- [Contents](#contents)
- [Install](#install)
- [Usage](#usage)
- [Configuration](#configuration)
- [Development](#development)
- [Team](#team)
- [License](#license)

## Install

Download the latest release from charon. 

## Usage

``` bash
charon init example --template typescript --directory . --repository https://github.com/perryrh0dan/example 
```

## Configuration

To configure taskline navigate to the ~/.charon.json file and modify any of the options to match your own preference. To reset back to the default values, simply delete the config file from your home directory.

The following illustrates all the available options with their respective default values.

``` json
{
  "templates_dir": "$HOME/.charon/templates",
  "templates_repo": {
    "url": "https://github.com/perryrh0dan/templates",
    "auth": "none",
    "token": null,
    "username": null,
    "password": null
  }
}
```

### In Detail

#### templates_dir
- Type: String
- Default: $HOME/.charon/templates

Filesystem path where all the templates are stored.

#### templates_repo
- Type: String
- Default: $HOME/.charon/templates

##### url
- Type: String
- Default: $HOME/.charon/templates

Url of the repository where templates are managed.

##### auth
- Type: String
- Default: none
- Values: `none`, `token`

##### token
- Type: String
- Default: none

Access token is only used when auth type is token

##### username
- Type: String
- Default: none

Coming soon

##### password
- Type: String
- Default: none

##### privatekey

Coming soon

##### 

## Development

## Team

- Thomas Pöhlmann ((@perryrh0dan))[https://github.com/perryrh0dan]

## License

(MIT)[https://github.com/perryrh0dan/charon/blob/master/license.md]