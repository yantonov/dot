[![Build Actions Status](https://github.com/yantonov/dot/workflows/ci/badge.svg)](https://github.com/yantonov/dot/actions)

###  Dot

#### Idea
Put configuration files (dot files) into the version control system.  
To use saved files just create symbolic links to them.  
This tool provides you with all the required automation by a single command.  

#### Mechanics
Dot tool iterates over the current directory (including subdirectories) and creates symbolic links from ${HOME} to all files inside the current directory (source, target directories can be paramerized, HOME is the reasonable default).  

Backup files will be generated before the creation of symbolic links (to preserve the existing content of the files that may be different than those you want to link).  
Why you need backup: by default, on the fresh system, for example, it's possible that you already have some config files, the idea is to prevent missing any existing information.

No specific layout (directory structure) or additional configuration files are required, just run the program inside the directory which contains your dot files.  

```
dot link
```

#### Installation
1. Put the binary ([latest release](https://github.com/yantonov/dot/releases/latest)) into the PATH
2. Use it from any directory (from your git repository or any other folder)

#### Windows specifics
You must have additional privileges to create symbolic links.
You can set permissions here:
```
Configuration \ Windows Settings \ Security Settings \ Local Policies \ User Rights Assignment\ Create symbolic links
```
By default Administrator can create symbolic links.
Make sure you run Git bash | Power shell | Cmd as administrator.

##### Examples
1. Bash files:  
[common unix settings](https://github.com/yantonov/sh/tree/master/nix/dotfiles)  
[ubuntu settings](https://github.com/yantonov/sh/tree/master/ubuntu/dotfiles)  
[mac settings](https://github.com/yantonov/sh/tree/master/mac/dotfiles)  
Common files are located in a separate directory. Platform specifics - inside the corresponding folder.
2. [nested directory example](https://github.com/yantonov/lein-conf/tree/master/dotfiles)  
Run the dot tool inside the dotfiles directory.

##### Some tecnhical notes: 
1. The app is distributed as a single binary to simplify the installation process.  
All logic is implemented in rust (only to provide a single binary), it helps to prevent any issues with scripting languages and setting environment (bash was not considered because long bash scripts are hard to maintain).
2. This implementation supports the simplest scenario to create symlinks, restore a regular file, and creating backup files before any symlink is introduced.
3. The composition of settings for different operation systems/platforms/devices can be done manually.
Corresponding settings can be versioned in separate directories as you like and after that, you can call this tool manually from the selected directories.
4. There is no goal to reimplement inside this tool some functionality of the version control system (to add/remove tracked files, checking changes, etc), or ansible (to distribute files across multiple devices/instances).
5. There is no goal to do anything with system files (under /etc/ for example) because it requires root access and the main intent is to support the simplest and the most typical case (and not deal with file permissions under the home directory).  
Update: you can try to use this tool for system files using --target parameter if you have corresponding permissions to write to the target directory.

##### Inspired by:
1. [missing-semester-course](https://github.com/missing-semester/missing-semester/blob/master/_2019/dotfiles.md)
2. [dotfiler](https://github.com/svetlyak40wt/dotfiler)
3. [dotfiles](https://github.com/holman/dotfiles)
4. [homesick](https://github.com/technicalpickles/homesick)
5. [dotfiles.github](https://dotfiles.github.io/)
6. [atlassian.dotfiles bare git repo](https://www.atlassian.com/git/tutorials/dotfiles) (nice, but it is hard to use dotfiles that are splitted into different repositories)
7. [stow](https://www.gnu.org/software/stow/) (nice, but  
a) I found it too late;  
b) there is no backup functionality;  
c) I wanted an interface that as simple as possible to cover the default case;  
d) stow has a little bit different ideology, for example about [tree-folding](https://www.gnu.org/software/stow/manual/stow.html#Tree-folding).  
Why tree-folding (symlinks for the directory) is not implemented: the common and specific settings like overrides can be stored in different directories\repositories while they are using the same directory structure, that's why it's not possible what target directory should be used for the symbolic link.
8. [lndir](https://linux.die.net/man/1/lndir) cannot handle relative directories, user experience is not so nice (not informative errors)  
9. [chezmoi](https://github.com/twpayne/chezmoi) too complicated for such trivial task, for multiple machines it's easier to have different folders (extract different parts and link it separately), than to have templates and code inside it (because it's hard to support files with multiple conditional statements inside it for different platforms).  

Thanks to all of these projects for the idea!
