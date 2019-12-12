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

# Gotchas and TODOs

- The bot-outputting backend needs to know what information
  players have. Note also that it would have to condition on
  the player it's playing for asymmetric games.
