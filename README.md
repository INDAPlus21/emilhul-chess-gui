# Chess GUI by Emil Hultcrantz

A playable chess GUI made using the [ggez](https://ggez.rs/) library. This program relies on the [murnion-chess](https://github.com/INDAPlus21/murnion-chess) chess engine by Felix Murnion for the underlying logic.

## Controls

Select a square using the mouse. If the square is a friendly piece a highlight of the squares to which it can move will be displayed. Choose one of those to move the piece to that square and pass the turn to your opponent.

The square on the right is used to to select which piece a pawn will promote to.

The history table on the right is clicked to choose a previous gamestate to view. The game will get a tint to show that your viewing the history. While in the past you can't do any moves only view how a piece was allowed to move on that turn. To go back to the present to continue playing the game click the most recent entry in the history table.

The program also uses two keybindings:

* The esc key exits the application
* The R key resets the chess game to the begining
