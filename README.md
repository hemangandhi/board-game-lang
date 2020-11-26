# What is this?

A programming language for boardgame game development.

# Why?

Games are complicated. Worse yet, players seem to get into the weeds
of rule-interaction before game developers figure them out. And this
raises tensions before coming to a logical resolution.

But, we're probably not far from the point where a computer can do
the dirty work up front and yell at the game developer as they're
developing the game. (Side-note: really, it would be 2 layers: the
structure should make some mistakes impossible and the machine
verification should take care of the rest.)

# Goal User

A game developer. That is, the creator of boardgames. There is no
illusion that they would know programming or be comfortable with
quantifiers or logic. The hope is that they are computer-oriented
and at least amenable to a robot overlord.

The rest of this doc is mostly written for programmers. Don't expect
to understand it without that background. There will be user docs
eventually.

# Features

## Minimal Viable Product

- An interactive UI that steadily builds game states.
- Verification about possible moves and an understanding of their interaction.
  - The computer will query about states it's uncertain about.
- Production of a rule book. (Of sorts.)

## Stage 2

- UI features:
  - Sprites/Meeples/Cards/other renderable stuff.
- Bots that play the game.

## Stage 3

- Web UI creation.

# The Project

- [] Design docs
- [] The programming system
- [] A soundness verifier (with meta-theorems, perhaps, of the theorems
     said verifier hopes to produce).
- [] Rule book output.
- [] Find and talk to people interested in boardgame creation to
     decide on strech goals. (Perhaps with some bitching on forums for
     games that are written up as test-cases.)

## Coding Details

This will be an open source Rust project. Why rust? Because it's the
easiest to parse in and universally deploy. Furthermore, I have the
best tooling knowledge for it (except python, but parsing there is a
headache).
