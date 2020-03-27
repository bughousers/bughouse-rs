# Bughousers

Logic API for [Bughouse Chess](https://en.wikipedia.org/wiki/Bughouse_chess) implemented in Rust

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

## Bughouse Chess

Rules for the Bughouse is different than normal chess so I implemented it with the rules for Bughouse from scratch, so the main differences:

+ You can deploy pieces on empty squares, with some restrictions
+ Pinned pieces can move
+ The King can move to an attacked square
+ The King can be captured

Meaning there are less legality checks and a checkmate does not terminate the game, also some illegal moves from the classic chess are legal in Bughouse.

## Usage

After calling the constructor the a bughouse game can be played only by calling the following functions:

The caller must specify the it's board and colour since there are two games running simultaneously.
There is a specific error in case if it is not the caller's turn.

To deploy a piece on a square call deploy_piece with your board,colour and the piece you want to deploy, if the piece cannot be deployed or does not exist in the pool, the function returns an error.
```rust
pub fn deploy_piece(&mut self,board1:bool,white:bool,p:Piece,i:usize,j:usize -> Result<bool,MoveError>
```
In case a pawn is to be promoted, the movemaker function checks a specific field for the promotion which can be a rook,knight,bishop or a queen. set_promotion has to be called with an appropriate piece before a promoting move occurs. The field are is after the promotion.
```rust
pub fn set_promotion(&mut self, board1:bool, p:Piece)
```
If you do not deploy a piece you need play a piece. The movemaker checks if the move is legal, then the function updates all the required fields and applies the move; otherwise, it returns an error.
The caller must specify the board and the location of the piece *(i_old,j_old)* and the new location of the piece *(i,j)* after the move.
The indices do not correspond the *SAN* notation. *(0,0)* is the upper-left corner (is the black rook) and *(7,7)* is the lower-right corner (is the white rook)
```rust
pub fn movemaker(&mut self, board1:bool, i_old:usize,j_old:usize,i:usize,j:usize) -> Result<bool,MoveError>
```

Sometimes you want to resign, so there is the function for a player to resign. It sets the winner field, and prohibits any moves after its call.
```rust
pub fn resign(&mut self,board1:bool,white:bool)
```
## Helpers

## Parse
A string of form *char*,*int*-*char*,*int* (e.g. *"e2-e4"*) can be converted to array indices via the parse function.

```rust
pub fn parse(input: &String) -> Option<[usize; 4]>
```
### Enum Piece
The Enum class of Piece has all the needed pieces, white and black pieces differ themselves: *P,Q,R,N,B,K,E,L,p,q,r,n,b* and their upgraded variants starting with *U*. The *U_* types are needed since when an upgraded piece is captured, your teammate receives a pawn.


## To-do
- [ ] Save calculation time by checking for a stalemate only after a request
- [ ] Chess clock support
- [ ] SAN support
