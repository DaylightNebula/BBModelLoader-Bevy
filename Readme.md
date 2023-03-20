This is meant to be a simple project that uses a few packages to make a game engine that behaves like a browser would.
By this I mean that the game engine can download code and assets from the online realm and run them as needed.

UI can easily be done with Belly.  A package that allows for simple creation of UI via .eml and .ess (basically just html and css) (https://github.com/jkb0o/belly/tree/main)

Goals:
 - Allow users to quickly create entities in json files with components and tasks.
 - Allow users to quickly create scenes using a collection of these entities.
 - Allow users to write custom components or entity tasks or scene behaviours using javascript.

Todolist:
- [x] File System
  - [x] Download file asynchronously
  - [x] Function to add to queue to download queue
    - [x] Should start download if higher and current queues are empty
    - [x] When a download is over, get first item of the highest queue to download
  - [x] Save to cache
  - [x] Queues should be stored in a hashmap that references an enum
- [ ] .scene files
  - [ ] entities array
    - [ ] If a string is given, load the path that it references
    - [ ] If a json object is given, create an entity with that json
  - [ ] uis array
    - [ ] If a string is given, load the path that it references
    - [ ] If a json object is given, create ui from that json
  - [ ] behaviour array
    - [ ] If a string is given, load the path that it references
    - [ ] If a json object is given, create a behaviour script from that json