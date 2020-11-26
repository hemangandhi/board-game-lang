# What?

An overview of the design of the processes and components that will for the board game creation system.

# High-level View

A pub-sub system with a central monitor managing a set of workers. There will also be a central database
for the game's state machine (ie. the PL abstractions).

# The Pub-sub System

Websockets.

There will be one public topic that all workers will use. The messages will mostly be updates to or from the UI.

# Workers

Each worker will probably be a thread in the monitoring program. Users would have to authenticate with the
monitoring program (if authentication is enabled).

## The PL Writer

This thread will read updates from the UI, parse them into the database representation, and insert the result.

### Open Questions

- Would the DB have a full revision history or a set of edits?
  - The PL Writer could maintain a running cumulative program while adding to a database of edits.
    Its message (or a shared object) would contain the cumulative program.

## The Question Generator

This thread will read the database representation and generate game states that the user could explore.

### Open Questions

- Should the questions be cached?
  - That way, perhaps there will be two workers: a question generator that will generate questions if the
    cache is empty and a question asker who'll try to empty the cache. This might be feasible in a thread,
    since a small cache size would be sensible given few users.

## Potential extensions

- Mutliple monitors

# Authentication

**Low-priority potential extension.**

If the server and UI are on different devices, this would be necessary. A random enough hash for a unique
room ID would suffice. The user would have to use that ID to connect to the server. The monitor would likely
manage this.