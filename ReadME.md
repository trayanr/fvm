# FVM - Flutter Version Manager
Enables the fast change of flutter versions

## Requires env vars
 * Adding FVM_PATH/smlink/flutter/bin to PATH
 * FVM_PATH is optional, defaults to 
    * /home/USER/.fvm on Linux

## Usage 
    fvm list [beta/dev/stable] : shows all versions availible for download of the channel 

    fvm download [version] : downloads the version

    fvm alias [version] [alias name] : adds and alias for the version so it can be accessed quickly

    fvm select [version/alias name] : changes the selected flutter version

    fvm show : list all installed versions of flutter

    fvm doctor : WIP
