# PawKit Filesystem

PawKit provides a Filesystem API, for handling game data.
It's split into two sections, a Read-Only API, and a Read-Write API.

## Read-Only API

The RO API is made up of `FilesystemSource`s, independent objects that handle their own files.
You're given a default source that represents the current working directory. You can create new sources from subdirectories or zip files from other sources. You can also create a new source from a raw zip file data.

## Read-Write API

The Read-Write API is based off of a virtual filesystem.
In native land, that corresponds 1:1 to the actual filesystem. In JS land, that will be implemented on top of IndexedDB.

The virtual filesystem is a global construct, and has no local instance.
