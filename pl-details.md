# What?

A description of the programming language.

Starts with theoretical reasoning about boardgames
and then moves on to real BNF and inference rules.

# Paradigm

- Strongly-typed
- Looks imperative, but implemented by everything
  living in a (sort of) monad (and programs just
  specify the set of states and then transitions).
  - This is why it might look somewhat "event-driven"
    as state transitions should trigger other state
    transitions.
- Players are sets of transitions.
- Rules restrict transitions and valid states.

## Example: Tichu

Code here is in psuedo-haskell.

```hs
data Suit = 0 | 1 | 2 | 3
data CurrentPlayer = 0 | 1 | 2 | 3
data Value = 2 | ... | A  -- Deriving Eq, Ord, and Enum might make sense
data Card = Suited Value Suit | Dog | One | Dragon | Pheonix

data Call = Grand | Tichu | NoCall

-- Assume Vec is a GADT `Nat -> a -> *` and I write Nats like numbers
data State = Game Int Int (Vec 4 $ Vec 14 $ Maybe Card)
               [Card] (Vec 4 Call) CurrentPlayer

play :: State -> State -> Bool
-- I'm too lazy to write this out
```

# Things that Should be Easy

- Adding enums: typed symbols.
- Defaults like decks, hands, and turns.

# The Grammar

(Meta-language: angle brackets are for non-terminal sets, `:=` is used for
definitions (the left hand side being some string I'm defining and the right
hand side being a non-definition string in the meta-language, and braces are
comma-separated lists of values with + being a suffix for one of more and *
for 0 or more. Backslashes are used to mean the literal versions of the 9
aforementioned symbols, in case I chose to use them.)

## Some Useful Non-Terminals and Assumptions

These are all regexes.

```
identifier := \w+
```

These are in the other meta-language:

```
built_in := {number,list of <type>}
```

Note that `<type>` is not purely a syntactic set: it is any type defined below.

Also, `<body>` is used to mean conditionals or action resolutions (function
calls).

Generally, statements in this language will end in a fullstop (`.`).

Furthermore, there will be support for arithmetic and some sort of lists.
The notion of lists may be convered by a special type and some functions.

## Type Declarations

Put `ident_phrase := {is <identifier> {or <identifier>}*, has a <type> called <identifier> {{and,with} a <type> called <identifier>}*}`.

```
A{n} <identifier> {of <identifier>} <ident_phrase>.
```

Comments will start with `Note:`.

### Examples

| Board Game Language | Haskell |
|---|---|
| `A die has a number called value.` | `data die = {value :: Int}`
| `A suit is spade or club or diamond or heart` | `data suit = spade | club | diamond | heart` |

## Conditionals

The hope is to create phrases that look like boardgame conditions.

```
if <matcher> then <body> else <body>
```

Matchers try to be phrases that make sense (thank God English is recursive,
huh). A table might make the correspondance clear:

| Boardgame lang | Most other programming languages |
|---|---|
| `<identifier> has <identifier> that {is,has} <matcher>` | `matcher(identifier.identifier)`|
| `<number> is {at least,bigger than,at most,smaller than} <number>` | `matcher(identifier.identifier)`|

## Function Declarations

These are mostly transitions in the game states.

```
Given a <type> called <identifier> {and a <type> called <identifier>}+,
resolve a{n} <identifier> to get a{n} <type> by <body>
```

The body may use `providing` to indicate returning, but this is to mostly to
make the language more natural (since players tend to get or lose things by
resolving actions).

### Example

```
Given a number called n resolve a factorial to get a number
by if n is at least 1 providing n * factorial (n - 1) else providing 1.
```

## Lists

```
An optional of stuff has a stuff called value and number called contents.
A list of stuff has an optional of stuff called head and list of stuff called tail.

Given a list of stuff called values resolve length to get a number
by if values has a head that has a contents that is 0 providing 0
   else providing 1 + length (the tail of values)
```

# Gotchas and TODOs

- The bot-outputting backend needs to know what information
  players have. Note also that it would have to condition on
  the player it's playing for asymmetric games.
- Are the turns and notion of players built-in?
  - What other built-ins are there?
  - Are decks, hands, and turns better off as libraries?
    - Also, since some games to let players interrupt turns.
