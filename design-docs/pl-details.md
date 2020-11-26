o# What?

A description of the programming language.

Starts with theoretical reasoning about boardgames
and then moves on to real BNF and inference rules.

# Paradigm

It doesn't matter what the PL looks like since it won't
really be user-facing (though as a debugging convenience,
it'll likely just be some lisp-y specification of some
record types, enums, and transition rules thereof). The
PL is mostly about storing a DAG and checking if the DAG
leads to a winning state.

The PL will be synthesized from interactions the board game
creator has with a virtual version of the game. Hence, the
abstactions that the UI can understand will be the limit of
the PL's expressivity: that which cannot be stated in the UI
will not be synthesized. Hence, UI components will correspond
to syntactic forms in the programming language. As a result,
this doc will describe the capacity of the UI.

## Rough Architecture

This will be fleshed out into a real web-app architecture
and in that fleshing out, there will be consideration of the
alternatives.

Roughly, however, it seems that a web-app will the be simplest
UI since it is natural to include image assets and drag and
drop interactions are straightforwardly supported. In fact,
even 3D graphics are somewhat tenable in a boardly usable way.

As more of the soundness-checking algorithm is understood, web-apps
can also handle the asyncronicity of time-consuming algorithms.
Furthermore, given the right amount of tooling, this can allow
for the rudimentary support of multi-player game testing.

The [web architecture](design-docs/web-architecture.md) has more on this.

# Objects

We can expect game objects to abstractly look like:

- Some user-provided image (low-priority)
- Numeric fields
- Fields drawn from an enumeration (strings that optionally map to images)
- Objects this object may contain (ie. a deck of cards, where each card
  is another object).

The notion of choice and who has that choice will be determined by the
interactions.

# Interactions

This determines how objects move around in the game. It will be done
by dragging and dropping. This means that various areas of the board
would have to be container objects (for discard piles, hands, and so
on).

Interactions will define an edge in the game state. The key idea is to
be able to abstract the example scenario that the game creator may be
dragging and dropping through. The UI will try this through asking what
fields in the object just dragged (and dropped to, perhaps) matter.
Integers and enumerations with equality should suffice for all the
predicates necessary for most games. Containers would need sizing
and index-based predicates too (consider sudoku-like constraints).

## Specifying Turns

This will probably be done be specifying the subject who is dragging
the object.

Most likely, there will be a subject representing the game rules in case
objects interact.

## Specifying Information Known by Players

This will probably be special field modifiers on the object or container.
Containers should be able to modify the visibility of their contents
(forcing a homogeniety of containers to some extent). An example is in
most hidden-hand card games: any card's data is visible when the card
is in a container the player can see (likely their hand and/or a public
deck).

Most simply, this can be an extra UI component next to any field.
For verification, if an edge predicate depends on the value of a field,
the visibility of that field will have to allow that edge predicate to be
evaluated.

# Gotchas and TODOs

- The bot-outputting backend needs to know what information
  players have. Note also that it would have to condition on
  the player it's playing for asymmetric games.
  - Special types for these later on?
- Are the turns and notion of players built-in?
  - What other built-ins are there?
  - Are decks, hands, and turns better off as libraries?
    - Also, since some games to let players interrupt turns.
