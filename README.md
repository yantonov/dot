[![Build Status](https://travis-ci.com/yantonov/dot.svg?branch=master)](https://travis-ci.com/yantonov/dot)

###  Dot

#### Idea
Put configuration files (dot files) into version control system.  
To use saved files just create symbolic links to them.  
This tool provides you all required automation by the single command.  

#### Mechanics
Dot tool iterates over the current directory including subdirectories and creates symbolic links from ${HOME} to all files inside the current directory.  
Backup files will be generated before the creation of symbolic links.  

#### Installation
1. Put the binary file into PATH
2. Use it from any directory (from your git repository or any other folder)

#### Examples
1. [some bash files](https://github.com/yantonov/sh/tree/master/mac/dotfiles)  
Run the dot tool inside the given directory.
2. [git config](https://github.com/yantonov/gitconfig)  
Run the dot tool inside the config directory.

##### Some tecnhical notes: 
1. The app is implemented in rust to simplify the distribution process and remove any issues with scripting languages and setting environment.
2. The composition of settings for different platforms/devices can be done manually.
Corresponding settings can be versioned in separate directories as you like and after that, you can call utility manually from selected directories.
3. This implementation supports simplest scenario to create symlinks, restore regular file and creating backup files before any symlink is introduced.

##### Todo list:
1. add command to remove backup files

##### Inspired by:
1. [dotfiler](https://github.com/svetlyak40wt/dotfiler)
2. [dotfiles](https://github.com/holman/dotfiles)
3. [homesick](https://github.com/technicalpickles/homesick)
4. [missing-semester-course](https://github.com/missing-semester/missing-semester/blob/master/_2019/dotfiles.md)

Thanks to all of these projects for the idea!
