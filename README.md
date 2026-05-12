cram is a config file manager.

# Windows Compatibility

cram uses symlinks which are disabled by default in Windows. To enable them; [enable developer mode](https://learn.microsoft.com/en-us/windows/advanced-settings/developer-mode#enable-developer-mode)

# Usage

cram is a lightweight config manager, allowing you to manage your configs from a git repo.
To begin setup a directory; conventionally this is called dotfiles, although this is not required. Inside this folder place your config files realtive to where they should appear in your home folder.

For example if your dotfiles folder looks like this:
```
~/
`-- .dotfiles/
    |-- .vimrc
    |-- .zshrc
    `-- .config/
        `-- starship.toml
```

Your home folder would then contain the following symlinks:
```
~/
|-- .vimrc
|-- .zshrc
`-- .config/
    `-- starship.toml
```
Resulting in a home directory that looks like this:
```
~/
|-- .dotfiles/
|   |-- .vimrc
|   |-- .zshrc
|   `-- .config/
|       `-- starship.toml
|-- .vimrc
|-- .zshrc
`-- .config/
    `-- starship.toml
```
