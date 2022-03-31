## About this project
*nodejs-launcher* is a CLI tool written in Rust which enables launch configurations seamlessly for NodeJS applications. This is still a work-in-progress project therefore it's not recommended to use in production environments. Suggestions for improvements are welcomed (please create the issue ticket).

## Use cases
Common IDEs (VScode, Webstorm etc.) typically offer built-in debuggers which extensively take advantage of launch configurations. This espesially comes handy when there's a lot of environment variables to pass to a NodeJS script. 

When IDE is not an option or lightweight alternative is preferred (a terminal editor, i.e. `vim`, `emacs`, `nano`), this simple CLI tool enables configuration presets for launching your nodejs apps and scripts with specified environment variables, arguments etc. 

The nodejs-launcher configuration has similar structure to VScode's built-in launch config (`launch.json`).

## CLI usage

`nodelauncher [command]`

Commands & options:

`init` - inits the config directory (by default `.node_launcher`) with config file `launch.json`

`run` - prompts user to select and execute one of available configurations specified in `launch.json` config file.

`edit` - opens terminal editor (nano) for changing available configurations

`add` - adds new configuration and opens terminal editor (nano) for configuring

## Launch configuration file *launch.json*

Launch config is a way for declaring every aspect of running the nodejs application. For instance, you can specify attributes, environment variables, enable debug mode and reuse different presents between launches.

## Launch configuration attributes

The following attributes are supported:

* `name` - the name of configuration which is used to reference and launch the particular configuration.

* `env` - a dictionary of key-value pairs which are passed as environment variables to executable nodejs script and can be accessed by referencing `process.env` in your script file.

* `script` - path to the script file which should execute with all environment variables specified in `env` JSON attribute,

