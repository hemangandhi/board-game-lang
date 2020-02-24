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

- A strongly-typed event-driven programming language that defines
  the game-play of a board game.
- A compiler that outputs a proof of the soundness of game rules.
- A compiler that outputs a rulebook (and perhaps card texts).
- (Slight stretch.) A compiler that outputs website source-code
  for an online version of the game.
  - (Stretching the stretch) A tree-based turn reviewing system.
- (Huge stretch.) A compiler that outputs an intelligent player
  (perhaps artifically so).

Dreams (as if the above weren't):

- The UI for the programming language will be block-based so that
  game developers are forced to write well-formed code (there should
  be an overriding mechanic).

# The Project

- [] Design docs
- [] The programming language
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
