## About this project
*nodejs-launcher* is a CLI tool written in Rust which enables launch configurations for NodeJS applications.

## Use cases
Common IDEs (VScode, Webstorm etc.) typically offer built-in debuggers which extensively take advantage of launch configurations. This espesially comes handy when there's a lot of environment variables to pass to a NodeJS script. 

In case you don't use an IDE or prefer a lightweight alternative (like a terminal editor, i.e. `vim`, `emacs`, `nano`) this tool enables launch configuration presets for your nodejs apps which takes away the annoying part of specifying environment variables or arguments manually. The nodejs-launcher configuration has similar structure to VScode's built-in configuration (`launch.json`).

## CLI commands

`nodelauncher --init` 
`nodelauncher -i` - inits the config directory (by default `.node_launcher`) with config file `launch.json`

`nodelauncher run --name <configuration_name>`
`nodelauncher run -n <configuration_name>` - executes configuration by its name, which is specifies in `launch.json` file

## Launch configuration file *launch.json*

Launch config is a way for declaring every aspect of running the nodejs application. For instance, you can specify attributes, environment variables or enable debug mode and reuse different presents between launches.

## Launch configuration attributes

The following attributes are supported:

* `name` - the name of configuration which is used to reference a particular configuration within a single script execution.

* `env` - a dictionary of key-value pairs which are passed as environment variables to executable nodejs script and can be accessed throu `process.env` within a nodejs runtime.

* `args` - a dictionary of key-value pairs which are passed as arguments to executable nodejs script. Essentially these are accessed through `process.argv` within a nodejs runtime.

* `executable` - the absolute path to nodejs runtime executable which is to be used. Default is `node`.

* `version` - the version of configuration template (for now, `0.1` version is set default and desn't need to be changed)
