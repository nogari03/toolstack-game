import './style.css'
import initChess, { GameEngine as ChessEngine } from 'chess-wasm'
import initJanggi, { GameEngine as JanggiEngine } from 'janggi-wasm'
import initMinesweeper, { GameEngine as MinesweeperEngine } from 'minesweeper-wasm'

type Color = "White" | "Black" | "Cho" | "Han";
type PieceType = "Pawn" | "Knight" | "Bishop" | "Rook" | "Queen" | "King" | "General" | "Advisor" | "Elephant" | "Horse" | "Chariot" | "Cannon" | "Soldier";

interface Piece {
  piece_type: PieceType;
  color: Color;
  has_moved: boolean;
}

type Square = Piece | null;
type BoardGrid = Square[][];

let engine: any;
let currentGame: "chess" | "janggi" | "minesweeper" = "chess";
let selectedSquare: { row: number, col: number } | null = null;
let validMovesForSelected: { row: number, col: number }[] = [];

const boardElement = document.getElementById('board')!;
const statusElement = document.getElementById('status')!;
const gameSelector = document.getElementById('game-selector') as HTMLSelectElement;
const resetButton = document.getElementById('reset-button') as HTMLButtonElement;

resetButton.addEventListener('click', () => {
  loadGame(currentGame);
});

// Disable context menu on board to support right click for flags
boardElement.addEventListener('contextmenu', (e) => {
  e.preventDefault();
});

const chessUnicodeMap: Record<string, Record<string, string>> = {
  White: { Pawn: '♙', Knight: '♘', Bishop: '♗', Rook: '♖', Queen: '♕', King: '♔' },
  Black: { Pawn: '♟', Knight: '♞', Bishop: '♝', Rook: '♜', Queen: '♛', King: '♚' }
};

const janggiUnicodeMap: Record<string, Record<string, string>> = {
  Han: { General: '漢', Advisor: '士', Elephant: '象', Horse: '馬', Chariot: '車', Cannon: '包', Soldier: '兵' },
  Cho: { General: '楚', Advisor: '士', Elephant: '象', Horse: '馬', Chariot: '車', Cannon: '包', Soldier: '卒' }
};

function renderBoard() {
  boardElement.innerHTML = '';

  const gameStatus: string = engine.get_status();

  if (currentGame === "minesweeper") {
    renderMinesweeperBoard(gameStatus);
    return;
  }

  const boardState: BoardGrid = JSON.parse(engine.get_board_state());
  const currentTurnInfo: Color = JSON.parse(engine.get_current_turn());

  const numRows = boardState.length;
  const numCols = boardState[0].length;

  boardElement.style.gridTemplateColumns = `repeat(${numCols}, 60px)`;
  boardElement.style.gridTemplateRows = `repeat(${numRows}, 60px)`;
  boardElement.style.border = "4px solid #333";
  boardElement.style.backgroundColor = "";

  if (gameStatus === "Active") {
    statusElement.textContent = `Turn: ${currentGame === 'janggi' ? (currentTurnInfo === 'White' ? 'HAN (Red)' : 'CHO (Blue/Green)') : currentTurnInfo.toUpperCase()} - Verified by Rust Engine 🦀`;
    resetButton.style.display = 'none';
  } else {
    statusElement.innerHTML = `<strong style="color:var(--focus-color); font-size: 1.5rem;">🚨 Game Over - ${gameStatus} 🚨</strong>`;
    resetButton.style.display = 'block';
  }

  for (let row = 0; row < numRows; row++) {
    for (let col = 0; col < numCols; col++) {
      const square = document.createElement('div');

      const isLight = currentGame === 'chess' ? (row + col) % 2 === 0 : true; // Janggi board is typically one color
      square.className = `square ${isLight ? 'light' : 'dark'}`;

      if (currentGame === "janggi") {
        square.style.backgroundColor = "#eebb77"; // Wood tone
        square.style.border = "1px solid #7a5022";
      }

      if (selectedSquare?.row === row && selectedSquare?.col === col) {
        square.classList.add('selected');
      }

      const isTarget = validMovesForSelected.some(m => m.row === row && m.col === col);
      if (isTarget) {
        square.style.boxShadow = "inset 0 0 10px 5px rgba(0, 255, 0, 0.4)";
      }

      const piece = boardState[row][col];
      if (piece) {
        const charMap = currentGame === "chess" ? chessUnicodeMap : janggiUnicodeMap;
        square.textContent = charMap[piece.color]?.[piece.piece_type] || '?';

        if (currentGame === "janggi") {
          square.style.color = piece.color === "Han" ? "#d32f2f" : "#0288d1"; // Red for Han, Blue for Cho
          square.style.fontWeight = "bold";
        } else {
          square.style.color = "#000";
        }
      } else {
        square.textContent = '';
      }

      const currentRow = row;
      const currentCol = col;
      square.addEventListener('click', () => {
        if (gameStatus === "Active") {
          handleSquareClick(currentRow, currentCol, piece, isTarget, currentTurnInfo);
        }
      });
      boardElement.appendChild(square);
    }
  }
}

function renderMinesweeperBoard(gameStatus: string) {
  const boardState: any[][] = JSON.parse(engine.get_board_state());
  const numRows = boardState.length;
  const numCols = boardState[0].length;

  boardElement.style.gridTemplateColumns = `repeat(${numCols}, 40px)`;
  boardElement.style.gridTemplateRows = `repeat(${numRows}, 40px)`;
  boardElement.style.border = "8px solid #bdbdbd";
  boardElement.style.backgroundColor = "#bdbdbd";

  if (gameStatus === "Active") {
    statusElement.textContent = `Minesweeper - Find all the safe squares! 💣`;
    resetButton.style.display = 'none';
  } else {
    statusElement.innerHTML = `<strong style="font-size: 1.5rem;">🚨 Game Over - ${gameStatus} 🚨</strong>`;
    resetButton.style.display = 'block';
  }

  for (let row = 0; row < numRows; row++) {
    for (let col = 0; col < numCols; col++) {
      const square = document.createElement('div');
      const cell = boardState[row][col];

      square.className = 'square mine-cell';

      if (cell.state === "Hidden") {
        square.classList.add('mine-hidden');
      } else if (cell.state === "Flagged") {
        square.classList.add('mine-flagged');
        square.textContent = '🚩';
      } else if (cell.state === "Revealed") {
        square.classList.add('mine-revealed');
        if (cell.is_mine) {
          square.textContent = '💣';
          square.style.backgroundColor = 'red';
        } else if (cell.adjacent_mines > 0) {
          square.textContent = cell.adjacent_mines.toString();
          square.classList.add(`mine-count-${cell.adjacent_mines}`);
        }
      }

      const currentRow = row;
      const currentCol = col;
      square.addEventListener('click', (e) => {
        if (gameStatus === "Active") {
          engine.reveal(currentRow, currentCol);
          renderBoard();
        }
      });

      square.addEventListener('contextmenu', (e) => {
        e.preventDefault();
        if (gameStatus === "Active") {
          engine.toggle_flag(currentRow, currentCol);
          renderBoard();
        }
      });

      boardElement.appendChild(square);
    }
  }
}

function handleSquareClick(row: number, col: number, piece: Piece | null, isTarget: boolean, currentTurn: string) {
  if (isTarget && selectedSquare) {
    const msg = engine.move_piece(selectedSquare.row, selectedSquare.col, row, col);
    const response = JSON.parse(msg);

    if (!response.success) {
      console.error("Wasm rejected move: " + response.message);
    }

    selectedSquare = null;
    validMovesForSelected = [];
    renderBoard();
    return;
  }

  if (selectedSquare) {
    if (selectedSquare.row === row && selectedSquare.col === col) {
      selectedSquare = null;
      validMovesForSelected = [];
    } else if (piece && (piece.color === currentTurn || (currentGame === "janggi" && piece.color === (currentTurn === "Black" ? "Cho" : "Han")))) {
      selectedSquare = { row, col };
      validMovesForSelected = engine.get_valid_moves(row, col).map((m: any) => ({ row: m.row, col: m.col }));
    } else {
      selectedSquare = null;
      validMovesForSelected = [];
    }
  } else {
    if (piece && (piece.color === currentTurn || (currentGame === "janggi" && piece.color === (currentTurn === "Black" ? "Cho" : "Han")))) {
      selectedSquare = { row, col };
      validMovesForSelected = engine.get_valid_moves(row, col).map((m: any) => ({ row: m.row, col: m.col }));
    }
  }
  renderBoard();
}

async function loadGame(game: "chess" | "janggi" | "minesweeper") {
  currentGame = game;
  selectedSquare = null;
  validMovesForSelected = [];

  if (game === "chess") {
    engine = new ChessEngine();
  } else if (game === "janggi") {
    engine = new JanggiEngine();
  } else if (game === "minesweeper") {
    engine = new MinesweeperEngine();
  }
  renderBoard();
}

gameSelector.addEventListener('change', (e) => {
  const newGame = (e.target as HTMLSelectElement).value as "chess" | "janggi" | "minesweeper";
  loadGame(newGame);
});

async function start() {
  await initChess();
  await initJanggi();
  await initMinesweeper();
  loadGame("chess");
}

start();
