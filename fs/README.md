# PawKit Virtual Filesystem

PawKit provides a Virtual Filesystem API, for handling game data.
It's split into two sections, a Read-Only API, and a Read-Write API.

## Read-Only API

The RO API is made up of `FilesystemSource`s, independent objects that handle their own files.
You're given a default source that represents the current working directory. You can create new sources from subdirectories or zip files from other sources.

## Read-Write API

TODO.
