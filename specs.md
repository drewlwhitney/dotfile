# Purpose
Easily create a full backup of the current computer that can be cloned on other computers.
This includes
- Installed applications (including things like kanata which require setup)
- WSL settings and packages

# Commands
`dot` - base level command, like `git`
- `pac` - commands involving packages. This can be customized to work with any package manager, as long as it can save packages to a file and then install from that same file.
    - `download` - load packages from remote and install them.

    - `upload` - upload packages to remote.

    - `sync` - runs `download` then `upload`.

    - `exclude [package(s)]` - prevent the provided packages from being uploaded.
    
    - `reinclude [package(s)]` - inverse of `exclude`.

    - `add [name]` - add a package manager to the package manager file. This should place in all of the required fields 

- `git` - commands involving **git**
    - `init [remote URL]` - initialize a new repository in the current directory and connect it to the provided remote

    - `pull` - pull changes from remote. On failure, this should notify the user and ask if they want to forcefully override the local state with the remote state.

    - `push [commit message]` - push all changes to remote with the provided commit message. Under the hood this runs `git add *`, `git commit -am [commit message]`, and `git push`. On failure, this should notify the user and ask if they want to forcefully override the remote state with the local state.

    - `sync` - runs `pull` and `push`

    - `ignore [file(s)]` - add the provided filenames (by their full path) to the .gitignore file.

    - `readd [file(s)]` - inverse of `ignore`.

- `file` - commands for working with actual files/folders
    - `add [file(s)]` - move the provided files to the **files** directory, then create symbolic links. Should check if the file is already present.

    - `remove [file(s)]` - remove the symbolic link, then move the file back to it's original place.

    - `purge [file(s)]` - remove the symbolic link and trash the file.

- `setup` - commands for working with application setup/other tasks
    - `add [name (s)]` - add a task setup entry
        - This creates a file in the **setup** directory that the user can configure. You must provide bash commands to run when setting up the application/running the task. Recall that bash can start a PowerShell instance if needed.

    - `remove [name(s)]` - trash the provided task entry.

    - `run [name(s) (optional)]` - run the application setup entry/task associated with **name**. If **name** is not provided, run all tasks.

- `fullsync` - runs `pac sync`, `pac sync -w`, `git sync`

- `new-system` - this assumes you have just pulled a repository from remote and want to set up your system. Runs `pac download`, `pac download -w`, links all files, runs `task run`.