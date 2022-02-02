## About this project
*nodejs-launcher* is a CLI tool written in Rust which enables launch configurations for NodeJS applications.

## Use cases
Common IDEs (VScode, Webstorm etc.) typically offer built-in debuggers which extensively take advantage of launch configurations. This espesially comes handy when there's a lot of environment variables to pass to a NodeJS script. 

In case you don't use an IDE or prefer a lightweight alternative (like a terminal editor, i.e. `vim`, `emacs`, `nano`) this tool is for you. It enables launch configuration presets for your nodejs apps by letting you specify environment variables, arguments etc. The nodejs-launcher configuration has similar structure to VScode's built-in configuration (`launch.json`).

## CLI usage

`nodelauncher [command] [options]`

Commands & options:

`init, i` - inits the config directory (by default `.node_launcher`) with config file `launch.json`

`run --name <name>, run -n <name>` - executes configuration by name specified in `launch.json` config file

`list, ls` - outputs all available launch configurations in current workspace

## Launch configuration file *launch.json*

Launch config is a way for declaring every aspect of running the nodejs application. For instance, you can specify attributes, environment variables, enable debug mode and reuse different presents between launches.

## Launch configuration attributes

The following attributes are supported:

* `name` - the name of configuration which is used to reference and launch the particular configuration.

* `env` - a dictionary of key-value pairs which are passed as environment variables to executable nodejs script and can be accessed by referencing `process.env` in your script file.

* `version` - the version of configuration template (for now, `0.1` version is set default and doesn't need to be changed)

