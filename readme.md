<h1 align="center">
    Tmpo
</h1>

<h4 align="center">
    Command line interface to quickly create new workspaces based on templates
</h4>

<div align="center">
  <img alt="rustacean" width="40%" src="media/rustacean.svg"/>
</div>

<div align="center">
   <a href="https://travis-ci.com/perryrh0dan/tmpo">
      <img alt="Build Status" src="https://travis-ci.com/perryrh0dan/tmpo.svg?branch=master" />
   </a>
   <a href="https://codecov.io/gh/perryrh0dan/tmpo">
      <img src="https://codecov.io/gh/perryrh0dan/tmpo/branch/master/graph/badge.svg" />
   </a>
   <a href="https://codeclimate.com/github/perryrh0dan/tmpo/maintainability">
      <img src="https://api.codeclimate.com/v1/badges/83561b59422e2492f9db/maintainability" />
   </a>
   <a href="https://gitter.im/perryrh0dan/tmpo/community">
      <img alt="Gitter" src="https://badges.gitter.im/taskline/community.svg" />
   </a>
</div>

## Description

Tmpo enables you to effectively create new workspaces based on predefined templates. To share templates across your team tmpo is using a git repository to manage your templates. Templates are automaticly fetched and merged on all clients. 

<div align="center">
  <img alt="diagram" width="100%" src="./media/diagram.png"/>
</div>

Visit the [contributing guidelines](https://github.com/perryrh0dan/tmpo/blob/master/contributing.md) to learn more on how help developing tmpo.

Come over to [Twitter](https://twitter.com/perryrh0dan1) to share your thoughts on the project.

## Highlights
- Setup complex workspaces in seconds
- Templates support inheritence and placeholders
- Lightweight & fast
- Share templates across your team or community with git
- Configurable through `~/.tmpo/config.yaml`

## Contents

- [Description](#description)
- [Highlights](#highlights)
- [Contents](#contents)
- [Demo](#demo)
- [Install](#install)
- [Usage](#usage)
- [Configuration](#configuration)
- [Development](#development)
- [Team](#team)
- [License](#license)

## Demo

<div align="center">
  <img alt="demo" width="70%" src="./media/demo_v1.0.0.gif">
</div>

## Install

1. Download the latest release for your platform.
2. Copy the binary to your `/bin` folder or point the path variable to it.
3. Add a repository with `tmpo repository add` and follow the instructions on the screen.

## Usage

### CLI

``` bash
tmpo init example --repository default --template typescript --directory . --remote https://github.com/perryrh0dan/2fa
```

### Repositories

A repository is a folder that contains up to n different templates. A repository can be connected with a remote git repository. Templates are then automaticly synced. Repositories are located in the 'template_dir'.

To add a new repository use the `repository subcommand` and follow the instructions on the screen.

``` bash
tmpo repository add
```

### Templates

Templates can be configured with a `meta.json` in the template root. 
For a detailed description how to create and maintain templates have a look at the instructions and default templates [repository](https://github.com/perryrh0dan/templates)

## Configuration
The configuration file should only be changed by experienced users.

To configure tmpo navigate to the ~/.tmpo/config.yaml file and modify any of the options to match your own preference. To reset back to the default values, simply delete the config file from your home directory.

The following illustrates all the available options with their respective default values.

``` yaml
  templates_dir: /home/thomas/.tmpo/templates
  templates_repositories:
    - name: default
      description: this is the default template repository from tpoe
      git_options:
        enabled: true
        url: "https://github.com/perryrh0dan/templates"
        auth: none
        token: 
        username: ~
        password: ~
```

## Development

### Build

#### Binary

Tmpo is using [cross](https://github.com/rust-embedded/cross) to build cross platform.

``` bash
cross build --target x86_64-pc-windows-gnu
cross build --release --target x86_64-pc-windows-gnu
```

#### Flatpak

tbd


## Team

- Thomas Pöhlmann [(@perryrh0dan)](https://github.com/perryrh0dan)

## License

[MIT](https://github.com/perryrh0dan/tmpo/blob/master/license.md)
