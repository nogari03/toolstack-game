import './style.css'
import init, { GameEngine } from 'chess-wasm'

// Types mirroring the Wasm Rust definitions
type Color = "White" | "Black";
type PieceType = "Pawn" | "Knight" | "Bishop" | "Rook" | "Queen" | "King";

interface Piece {
  piece_type: PieceType;
  color: Color;
  has_moved: boolean;
}

type Square = Piece | null;
type BoardGrid = Square[][];

let engine: GameEngine;
let selectedSquare: { row: number, col: number } | null = null;
let validMovesForSelected: { row: number, col: number }[] = [];

const boardElement = document.getElementById('board')!;
const statusElement = document.getElementById('status')!;

// Unicode map for pieces
const pieceUnicodeMap = {
  White: { Pawn: '♙', Knight: '♘', Bishop: '♗', Rook: '♖', Queen: '♕', King: '♔' },
  Black: { Pawn: '♟', Knight: '♞', Bishop: '♝', Rook: '♜', Queen: '♛', King: '♚' }
};

function renderBoard() {
  boardElement.innerHTML = '';

  // Parse the board state and turn directly from Wasm
  const boardState: BoardGrid = JSON.parse(engine.get_board_state());
  const currentTurnInfo: Color = JSON.parse(engine.get_current_turn());

  statusElement.textContent = `Turn: ${currentTurnInfo.toUpperCase()} - Verified by Rust Engine 🦀`;

  for (let row = 0; row < 8; row++) {
    for (let col = 0; col < 8; col++) {
      const square = document.createElement('div');
      const isLight = (row + col) % 2 === 0;
      square.className = `square ${isLight ? 'light' : 'dark'}`;

      // Highlight currently selected piece
      if (selectedSquare?.row === row && selectedSquare?.col === col) {
        square.classList.add('selected');
      }

      // Highlight squares that are valid destinations
      const isTarget = validMovesForSelected.some(m => m.row === row && m.col === col);
      if (isTarget) {
        // A visual indicator for valid targets
        square.style.boxShadow = "inset 0 0 10px 5px rgba(0, 255, 0, 0.4)";
      }

      const piece = boardState[row][col];
      if (piece) {
        square.textContent = pieceUnicodeMap[piece.color][piece.piece_type];
      } else {
        square.textContent = '';
      }

      const currentRow = row;
      const currentCol = col;
      square.addEventListener('click', () => handleSquareClick(currentRow, currentCol, piece, isTarget, currentTurnInfo));
      boardElement.appendChild(square);
    }
  }
}

function handleSquareClick(row: number, col: number, piece: Piece | null, isTarget: boolean, currentTurn: string) {
  if (isTarget && selectedSquare) {
    // User clicked a valid destination tile for the selected piece
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
    // Deselect if same square
    if (selectedSquare.row === row && selectedSquare.col === col) {
      selectedSquare = null;
      validMovesForSelected = [];
    } else if (piece && piece.color === currentTurn) {
      // Select a different player piece
      selectedSquare = { row, col };
      validMovesForSelected = engine.get_valid_moves(row, col).map((m: any) => ({ row: m.row, col: m.col }));
    } else {
      // Clicked invalid empty spot or enemy piece when not a valid target
      selectedSquare = null;
      validMovesForSelected = [];
    }
  } else {
    // Select square if it has a piece and belongs to the current turn
    if (piece && piece.color === currentTurn) {
      selectedSquare = { row, col };
      validMovesForSelected = engine.get_valid_moves(row, col).map((m: any) => ({ row: m.row, col: m.col }));
    }
  }
  renderBoard();
}

async function start() {
  await init();
  engine = new GameEngine();
  renderBoard();
}

start();
