import './style.css'
import initChess, { GameEngine as ChessEngine } from 'chess-wasm'
import initChess2, { GameEngine as Chess2Engine } from 'chess2-wasm'
import initJanggi, { GameEngine as JanggiEngine } from 'janggi-wasm'
import initMinesweeper, { GameEngine as MinesweeperEngine } from 'minesweeper-wasm'
import init2048, { GameEngine as Game2048Engine } from 'game2048-wasm'
import initRacing, { RacingGame } from 'racing-wasm'
import initClaw, { ClawGame } from 'claw-wasm'
import { LeaderboardAPI } from './leaderboard'

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
let currentGame: "chess" | "chess2" | "janggi" | "minesweeper" | "2048" | "racing" | "claw" = "chess";
let currentMinesweeperDiff: "beginner" | "intermediate" | "expert" = "beginner";
let currentAiModeStatus: { enabled: boolean, depth: number } = { enabled: false, depth: 1 };
let selectedSquare: { row: number, col: number } | null = null;
let lastMove: { fr: number, fc: number, tr: number, tc: number } | null = null;
let validMovesForSelected: { row: number, col: number }[] = [];
let keys = { up: false, down: false, left: false, right: false, space: false };
let gameEndedRecorded: boolean = false;

let guestUserId = localStorage.getItem('guest_user_id') || '';
if (!guestUserId) {
  guestUserId = 'User-' + Math.floor(Math.random() * 10000).toString().padStart(4, '0');
  localStorage.setItem('guest_user_id', guestUserId);
}

const boardElement = document.getElementById('board')!;
const statusElement = document.getElementById('status')!;
const menuButtons = document.querySelectorAll('.menu-btn');
const resetButton = document.getElementById('reset-button') as HTMLButtonElement;
const leaderboardList = document.getElementById('leaderboard-list')!;
const themeToggleBtn = document.getElementById('theme-toggle') as HTMLButtonElement;
const syncBtn = document.getElementById('sync-leaderboard-btn') as HTMLButtonElement;

syncBtn.addEventListener('click', () => {
  alert("서비스 준비중 입니다");
});

// Theme toggle logic
let isDarkMode = localStorage.getItem('theme') === 'dark';
if (isDarkMode) {
  document.documentElement.setAttribute('data-theme', 'dark');
  themeToggleBtn.textContent = '☀️ Light Mode';
}

themeToggleBtn.addEventListener('click', () => {
  isDarkMode = !isDarkMode;
  if (isDarkMode) {
    document.documentElement.setAttribute('data-theme', 'dark');
    localStorage.setItem('theme', 'dark');
    themeToggleBtn.textContent = '☀️ Light Mode';
  } else {
    document.documentElement.removeAttribute('data-theme');
    localStorage.setItem('theme', 'light');
    themeToggleBtn.textContent = '🌙 Dark Mode';
  }
});

resetButton.addEventListener('click', () => {
  loadGame(currentGame);
});

// Disable context menu on board to support right click for flags
boardElement.addEventListener('contextmenu', (e) => {
  e.preventDefault();
});

// Key bindings for 2048 and Racing
window.addEventListener('keydown', (e) => {
  if (currentGame === "racing") {
    if (["ArrowUp", "w", "W"].includes(e.key)) keys.up = true;
    if (["ArrowDown", "s", "S"].includes(e.key)) keys.down = true;
    if (["ArrowLeft", "a", "A"].includes(e.key)) keys.left = true;
    if (["ArrowRight", "d", "D"].includes(e.key)) keys.right = true;
    // Prevent default scrolling
    if (["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"].includes(e.key)) e.preventDefault();
  }

  if (currentGame === "claw") {
    if (["ArrowLeft", "a", "A"].includes(e.key)) keys.left = true;
    if (["ArrowRight", "d", "D"].includes(e.key)) keys.right = true;
    if (e.key === " " || e.key === "Spacebar") {
      keys.space = true;
      e.preventDefault();
    }
    if (["ArrowLeft", "ArrowRight"].includes(e.key)) e.preventDefault();
  }

  if (currentGame === "2048" && engine) {
    const status = engine.get_status();
    if (status !== "Active") return;

    let direction = null;
    if (e.key === "ArrowUp" || e.key === "w" || e.key === "W") direction = "Up";
    if (e.key === "ArrowDown" || e.key === "s" || e.key === "S") direction = "Down";
    if (e.key === "ArrowLeft" || e.key === "a" || e.key === "A") direction = "Left";
    if (e.key === "ArrowRight" || e.key === "d" || e.key === "D") direction = "Right";

    if (direction) {
      e.preventDefault();
      engine.execute_move(direction);
      renderBoard();
    }
  }
});

window.addEventListener('keyup', (e) => {
  if (currentGame === "racing") {
    if (["ArrowUp", "w", "W"].includes(e.key)) keys.up = false;
    if (["ArrowDown", "s", "S"].includes(e.key)) keys.down = false;
    if (["ArrowLeft", "a", "A"].includes(e.key)) keys.left = false;
    if (["ArrowRight", "d", "D"].includes(e.key)) keys.right = false;
  }

  if (currentGame === "claw") {
    if (["ArrowLeft", "a", "A"].includes(e.key)) keys.left = false;
    if (["ArrowRight", "d", "D"].includes(e.key)) keys.right = false;
    if (e.key === " " || e.key === "Spacebar") keys.space = false;
  }
});

const chessUnicodeMap: Record<string, Record<string, string>> = {
  White: { Pawn: '♙', Knight: '♘', Bishop: '♗', Rook: '♖', Queen: '♕', King: '♔' },
  Black: { Pawn: '♟', Knight: '♞', Bishop: '♝', Rook: '♜', Queen: '♛', King: '♚' }
};

const chessPieceImages: Record<string, Record<string, string>> = {
  White: { Pawn: '/pieces/chess2/w-pawn.png', Knight: '/pieces/chess2/w-knight.png', Bishop: '/pieces/chess2/w-bishop.png', Rook: '/pieces/chess2/w-rook.png', Queen: '/pieces/chess2/w-queen.png', King: '/pieces/chess2/w-king.png' },
  Black: { Pawn: '/pieces/chess2/b-pawn.png', Knight: '/pieces/chess2/b-knight.png', Bishop: '/pieces/chess2/b-bishop.png', Rook: '/pieces/chess2/b-rook.png', Queen: '/pieces/chess2/b-queen.png', King: '/pieces/chess2/b-king.png' }
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

  if (currentGame === "2048") {
    render2048Board(gameStatus);
    return;
  }

  if (currentGame === "racing" || currentGame === "claw") {
    return; // Racing and Claw have their own animation loop
  }

  const boardState: BoardGrid = JSON.parse(engine.get_board_state());
  const currentTurnInfo: Color = JSON.parse(engine.get_current_turn());

  const numRows = boardState.length;
  const numCols = boardState[0].length;

  boardElement.style.display = 'grid';
  boardElement.style.gridTemplateColumns = `repeat(${numCols}, 60px)`;
  boardElement.style.gridTemplateRows = `repeat(${numRows}, 60px)`;
  boardElement.style.border = "4px solid #333";
  boardElement.style.backgroundColor = "";
  boardElement.style.gap = "0px";
  boardElement.style.padding = "0px";
  boardElement.style.width = "auto";
  boardElement.style.height = "auto";

  if (gameStatus === "Active") {
    statusElement.textContent = `Turn: ${currentGame === 'janggi' ? (currentTurnInfo === 'White' ? 'HAN (Red)' : 'CHO (Blue/Green)') : currentTurnInfo.toUpperCase()}`;
    resetButton.style.display = 'none';
  } else {
    statusElement.innerHTML = `${gameStatus}`;
    resetButton.style.display = 'block';

    if (!gameEndedRecorded) {
      gameEndedRecorded = true;
      let result = "Draw";
      if (gameStatus.includes("wins") || gameStatus.includes("Win")) result = "Win";
      else if (gameStatus.includes("mate")) result = "Lose";

      LeaderboardAPI.saveRecord({
        game: currentGame,
        userId: guestUserId,
        result: result,
        date: new Date().toISOString()
      }).then(() => updateLeaderboardDisplay());
    }
  }

  for (let row = 0; row < numRows; row++) {
    for (let col = 0; col < numCols; col++) {
      const square = document.createElement('div');

      const isLight = (currentGame === 'chess' || currentGame === 'chess2') ? (row + col) % 2 === 0 : true; // Janggi board is typically one color
      square.className = `square ${isLight ? 'light' : 'dark'}`;

      if (currentGame === "janggi") {
        square.style.backgroundColor = "#eebb77"; // Wood tone
        square.style.border = "1px solid #7a5022";
      }

      if (lastMove && (lastMove.tr === row && lastMove.tc === col)) {
        square.style.backgroundColor = currentGame === "janggi" ? "#cfa964" : "#f1c40f";
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
        if (currentGame === "chess2") {
          square.innerHTML = `<img src="${chessPieceImages[piece.color]?.[piece.piece_type]}" style="width: 80%; height: 80%; pointer-events: none;" />`;
          square.style.display = 'flex';
          square.style.justifyContent = 'center';
          square.style.alignItems = 'center';
        } else {
          const charMap = currentGame === "chess" ? chessUnicodeMap : janggiUnicodeMap;
          square.textContent = charMap[piece.color]?.[piece.piece_type] || '?';

          if (currentGame === "janggi") {
            square.style.color = piece.color === "Han" ? "#d32f2f" : "#0288d1"; // Red for Han, Blue for Cho
            square.style.fontWeight = "bold";
          } else {
            square.style.color = "#000";
          }
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

function render2048Board(gameStatus: string) {
  const boardState: number[][] = JSON.parse(engine.get_board_state());
  const score = engine.get_score();

  boardElement.style.display = 'grid';
  boardElement.style.gridTemplateColumns = `repeat(4, 100px)`;
  boardElement.style.gridTemplateRows = `repeat(4, 100px)`;
  boardElement.style.border = "none";
  boardElement.style.backgroundColor = "#bbada0";
  boardElement.style.gap = "10px";
  boardElement.style.padding = "10px";
  boardElement.style.borderRadius = "8px";
  boardElement.style.width = "auto";
  boardElement.style.height = "auto";

  if (gameStatus === "Active") {
    statusElement.innerHTML = `2048 - Use Arrow Keys to move tiles! <br> <strong>Score: ${score}</strong>`;
    resetButton.style.display = 'none';
  } else {
    statusElement.innerHTML = `<strong style="font-size: 1.5rem;">${gameStatus}<br> Final Score: ${score}</strong>`;
    resetButton.style.display = 'block';

    if (!gameEndedRecorded) {
      gameEndedRecorded = true;
      LeaderboardAPI.saveRecord({
        game: currentGame,
        userId: guestUserId,
        result: gameStatus.includes("Won") ? "Win" : "Lose",
        score: score,
        date: new Date().toISOString()
      }).then(() => updateLeaderboardDisplay());
    }
  }

  for (let row = 0; row < 4; row++) {
    for (let col = 0; col < 4; col++) {
      const square = document.createElement('div');
      const val = boardState[row][col];

      square.className = 'square game-2048-cell';
      if (val > 0) {
        square.textContent = val.toString();
        square.classList.add(`val-${val}`);
      }

      boardElement.appendChild(square);
    }
  }
}

function renderMinesweeperBoard(gameStatus: string) {
  // If no engine, it means we are at the main menu, rendering is handled there
  if (!engine) return;

  // ... existing code, just reset the gap/padding if we switch back
  boardElement.style.gap = "0px";
  boardElement.style.padding = "0px";
  boardElement.style.width = "auto";
  boardElement.style.height = "auto";

  const boardState: any[][] = JSON.parse(engine.get_board_state());
  const numRows = boardState.length;
  const numCols = boardState[0].length;

  boardElement.style.display = 'grid';
  boardElement.style.gridTemplateColumns = `repeat(${numCols}, 40px)`;
  boardElement.style.gridTemplateRows = `repeat(${numRows}, 40px)`;
  boardElement.style.border = "8px solid #bdbdbd";
  boardElement.style.backgroundColor = "#bdbdbd";

  if (gameStatus === "Active") {
    statusElement.textContent = `Minesweeper (${currentMinesweeperDiff})`;
    resetButton.style.display = 'none';
  } else {
    statusElement.innerHTML = `<strong style="font-size: 1.5rem;">${gameStatus}</strong>`;
    resetButton.style.display = 'block';

    if (!gameEndedRecorded) {
      gameEndedRecorded = true;
      let recordGameName = `minesweeper-${currentMinesweeperDiff}`;
      LeaderboardAPI.saveRecord({
        game: recordGameName,
        userId: guestUserId,
        result: gameStatus === "Won" ? "Win" : "Lose",
        date: new Date().toISOString()
      }).then(() => updateLeaderboardDisplay());
    }
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
      square.addEventListener('click', () => {
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

let racingAnimationFrameId: number | null = null;
let racingCanvas: HTMLCanvasElement | null = null;
let racingCtx: CanvasRenderingContext2D | null = null;

let clawAnimationFrameId: number | null = null;
let clawCanvas: HTMLCanvasElement | null = null;
let clawCtx: CanvasRenderingContext2D | null = null;

function stopClawLoop() {
  if (clawAnimationFrameId !== null) {
    cancelAnimationFrame(clawAnimationFrameId);
    clawAnimationFrameId = null;
  }
  if (clawCanvas && clawCanvas.parentElement) {
    clawCanvas.parentElement.removeChild(clawCanvas);
  }
  clawCanvas = null;
  clawCtx = null;
  keys.space = false; keys.left = false; keys.right = false;
}

function startClawLoop() {
  boardElement.innerHTML = '';
  boardElement.style.display = 'block';
  boardElement.style.border = '4px solid #333';
  boardElement.style.padding = '0';
  boardElement.style.backgroundColor = '#ddd';
  boardElement.style.width = 'auto';
  boardElement.style.height = 'auto';

  clawCanvas = document.createElement('canvas');
  clawCanvas.width = 800;
  clawCanvas.height = 600;
  clawCanvas.style.display = 'block';
  clawCtx = clawCanvas.getContext('2d')!;
  boardElement.appendChild(clawCanvas);

  statusElement.innerHTML = `Claw Machine - Use Left/Right arrows to move, Space to drop!<br><strong>Score: 0</strong>`;
  resetButton.style.display = 'block';

  function loop() {
    if (currentGame !== "claw" || !engine) return;
    engine.update(keys.left, keys.right, keys.space);
    renderClawBoard();
    clawAnimationFrameId = requestAnimationFrame(loop);
  }
  clawAnimationFrameId = requestAnimationFrame(loop);
}

function renderClawBoard() {
  if (!clawCtx || !engine) return;
  const state = JSON.parse(engine.get_state_json());

  // Background
  clawCtx.fillStyle = '#f0f4f8';
  clawCtx.fillRect(0, 0, state.width, state.height);

  // Drop zone (HOME_X area)
  clawCtx.fillStyle = '#cbd5e1';
  clawCtx.fillRect(0, 0, 150, state.height);
  clawCtx.fillStyle = '#94a3b8';
  clawCtx.fillRect(60, state.height - 100, 80, 100); // Hole

  // Toys
  for (const toy of state.toys) {
    clawCtx.fillStyle = toy.color;
    // Draw toy as a rounded rect or circle with a bow
    clawCtx.beginPath();
    clawCtx.arc(toy.x + toy.width / 2, toy.y + toy.height / 2, toy.width / 2, 0, Math.PI * 2);
    clawCtx.fill();

    // Ribbon
    clawCtx.fillStyle = '#ffffff';
    clawCtx.fillRect(toy.x + toy.width / 2 - 2, toy.y, 4, toy.height);
    clawCtx.fillRect(toy.x, toy.y + toy.height / 2 - 2, toy.width, 4);

    // Add value text if possible
    clawCtx.fillStyle = '#fff';
    clawCtx.font = "bold 12px Arial";
    clawCtx.textAlign = "center";
    clawCtx.fillText(`$${toy.value}`, toy.x + toy.width / 2, toy.y + toy.height / 2 + 4);
  }

  // Draw Claw Base Line
  clawCtx.strokeStyle = '#475569';
  clawCtx.lineWidth = 4;
  clawCtx.beginPath();
  clawCtx.moveTo(state.claw_x + 30, 0); // 30 is CLAW_WIDTH / 2
  clawCtx.lineTo(state.claw_x + 30, state.claw_y);
  clawCtx.stroke();

  // Draw Claw Grabber (Top box)
  clawCtx.fillStyle = '#fbbf24'; // Golden claw base
  clawCtx.fillRect(state.claw_x + 10, state.claw_y, 40, 20);

  // Draw Grabber Prongs
  clawCtx.strokeStyle = '#64748b'; // Silver prongs
  clawCtx.lineWidth = 6;

  let prongAngle = 0;
  if (state.claw_state === "Dropping" || state.claw_state === "Raising" || state.claw_state === "Returning" || state.claw_state === "Releasing") {
    prongAngle = 20; // Open
  } else if (state.claw_state === "Idle") {
    prongAngle = 20; // Open
  } else if (state.claw_state === "Grabbing") {
    prongAngle = 0; // Closed
  }

  // Left prong
  clawCtx.beginPath();
  clawCtx.moveTo(state.claw_x + 10, state.claw_y + 20);
  clawCtx.lineTo(state.claw_x - prongAngle, state.claw_y + 60);
  // hook inward
  clawCtx.lineTo(state.claw_x + 10 - prongAngle, state.claw_y + 80);
  clawCtx.stroke();

  // Right prong
  clawCtx.beginPath();
  clawCtx.moveTo(state.claw_x + 50, state.claw_y + 20);
  clawCtx.lineTo(state.claw_x + 60 + prongAngle, state.claw_y + 60);
  // hook inward
  clawCtx.lineTo(state.claw_x + 50 + prongAngle, state.claw_y + 80);
  clawCtx.stroke();

  statusElement.innerHTML = `Claw Machine - Use Left/Right arrows to move, Space to drop!<br><strong>Score: ${state.score}</strong>`;
}

function stopRacingLoop() {
  if (racingAnimationFrameId !== null) {
    cancelAnimationFrame(racingAnimationFrameId);
    racingAnimationFrameId = null;
  }
  if (racingCanvas && racingCanvas.parentElement) {
    racingCanvas.parentElement.removeChild(racingCanvas);
  }
  racingCanvas = null;
  racingCtx = null;
  keys.up = false; keys.down = false; keys.left = false; keys.right = false;
}

function startRacingLoop() {
  boardElement.innerHTML = '';
  boardElement.style.display = 'block';
  boardElement.style.border = '4px solid #333';
  boardElement.style.padding = '0';
  boardElement.style.backgroundColor = '#222';
  boardElement.style.width = 'auto';
  boardElement.style.height = 'auto';

  racingCanvas = document.createElement('canvas');
  racingCanvas.width = 800; // 20 tiles * 40px
  racingCanvas.height = 600; // 15 tiles * 40px
  racingCanvas.style.display = 'block';
  racingCtx = racingCanvas.getContext('2d')!;
  boardElement.appendChild(racingCanvas);

  statusElement.textContent = `Racing - Use WASD or Arrows to drive!`;
  resetButton.style.display = 'block';

  function loop() {
    if (currentGame !== "racing" || !engine) return;
    engine.update(keys.up, keys.down, keys.left, keys.right);
    renderRacingBoard();
    racingAnimationFrameId = requestAnimationFrame(loop);
  }
  racingAnimationFrameId = requestAnimationFrame(loop);
}

function renderRacingBoard() {
  if (!racingCtx || !engine) return;
  const state = JSON.parse(engine.get_state_json());

  // Draw track
  for (let y = 0; y < state.height; y++) {
    for (let x = 0; x < state.width; x++) {
      const tile = state.track[y][x];
      if (tile === "Grass") racingCtx.fillStyle = '#4caf50';
      else if (tile === "Road") racingCtx.fillStyle = '#9e9e9e';
      else racingCtx.fillStyle = '#607d8b'; // Wall

      racingCtx.fillRect(x * 40, y * 40, 40, 40);

      // Optional grid lines
      racingCtx.strokeStyle = 'rgba(0,0,0,0.1)';
      racingCtx.strokeRect(x * 40, y * 40, 40, 40);
    }
  }

  // Draw Car
  racingCtx.save();
  racingCtx.translate(state.car.x, state.car.y);
  racingCtx.rotate(state.car.angle);

  // Car chassis
  racingCtx.fillStyle = '#d32f2f'; // Red mini yonku style
  racingCtx.fillRect(-12, -8, 24, 16);

  // Windshield
  racingCtx.fillStyle = '#e0f7fa';
  racingCtx.fillRect(2, -5, 6, 10);

  // Headlights
  racingCtx.fillStyle = '#ffff00';
  racingCtx.fillRect(10, -7, 3, 3);
  racingCtx.fillRect(10, 4, 3, 3);

  // Tires
  racingCtx.fillStyle = '#212121';
  racingCtx.fillRect(-10, -11, 8, 3); // Back left
  racingCtx.fillRect(4, -11, 8, 3);   // Front left
  racingCtx.fillRect(-10, 8, 8, 3);   // Back right
  racingCtx.fillRect(4, 8, 8, 3);     // Front right

  racingCtx.restore();

  statusElement.innerHTML = `Racing - Speed: ${Math.abs(state.car.speed).toFixed(1)} km/h`;
}

function triggerAIMove() {
  const gameStatus: string = engine.get_status();
  if (gameStatus !== "Active") return;

  if (!currentAiModeStatus.enabled) return;

  statusElement.innerHTML = `<strong>AI is thinking...</strong>`;

  setTimeout(() => {
    const bestMoveStr = engine.get_best_move(currentAiModeStatus.depth);
    if (bestMoveStr !== "null") {
      const move = JSON.parse(bestMoveStr);
      lastMove = { fr: move.fr, fc: move.fc, tr: move.tr, tc: move.tc };
      engine.move_piece(move.fr, move.fc, move.tr, move.tc);
      renderBoard();
    }
  }, 50);
}

function handleSquareClick(row: number, col: number, piece: Piece | null, isTarget: boolean, currentTurn: string) {
  if (isTarget && selectedSquare) {
    const msg = engine.move_piece(selectedSquare.row, selectedSquare.col, row, col);
    const response = JSON.parse(msg);

    if (!response.success) {
      console.error("Wasm rejected move: " + response.message);
    } else {
      lastMove = { fr: selectedSquare.row, fc: selectedSquare.col, tr: row, tc: col };
    }

    selectedSquare = null;
    validMovesForSelected = [];
    renderBoard();

    if (currentAiModeStatus.enabled && response.success) {
      const newTurnInfo = JSON.parse(engine.get_current_turn());
      if (((currentGame === "chess" || currentGame === "chess2") && newTurnInfo === "Black") ||
        (currentGame === "janggi" && newTurnInfo === "White")) {
        triggerAIMove();
      }
    }

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

async function updateLeaderboardDisplay() {
  if (!leaderboardList) return;
  let queryGameId = currentGame as string;
  if (currentGame === "minesweeper") {
    queryGameId = `minesweeper-${currentMinesweeperDiff}`;
  }
  const records = await LeaderboardAPI.getRecords(queryGameId);
  leaderboardList.innerHTML = '';

  if (records.length === 0) {
    leaderboardList.innerHTML = '<div style="padding: 15px; text-align: center; color: #888;">No records yet. Be the first!</div>';
    return;
  }

  records.slice(0, 10).forEach((record, index) => {
    const row = document.createElement('div');
    row.className = `notice-row notice-item rank-${index + 1}`;

    let scoreText = record.score !== undefined ? record.score.toString() : record.result;

    row.innerHTML = `
      <span class="rank-col">${index + 1}</span>
      <span class="user-col">${record.userId}</span>
      <span class="score-col">${scoreText}</span>
    `;

    // Highlight wins visually
    if (record.result === "Win") {
      row.style.background = "rgba(76, 175, 80, 0.1)"; // Subtle green tint for win
    }

    leaderboardList.appendChild(row);
  });
}

function showMinesweeperMenu() {
  boardElement.style.display = 'flex';
  boardElement.style.flexDirection = 'column';
  boardElement.style.justifyContent = 'center';
  boardElement.style.alignItems = 'center';
  boardElement.style.gap = '20px';
  boardElement.style.width = '400px';
  boardElement.style.height = '400px';
  boardElement.style.backgroundColor = 'var(--win-window-bg)';
  boardElement.style.border = '4px solid var(--win-border-light)';
  boardElement.style.boxShadow = 'inset 1px 1px var(--win-border-dark), inset -1px -1px var(--win-border-white)';

  statusElement.textContent = `Select Difficulty`;
  resetButton.style.display = 'none';

  boardElement.innerHTML = `
    <h3 style="margin: 0; color: var(--win-text);">Select Difficulty</h3>
    <button class="ms-diff-btn" data-diff="beginner" style="width: 250px; text-align: center; border: 2px solid; border-color: var(--win-border-white) var(--win-border-dark) var(--win-border-dark) var(--win-border-white); background: var(--win-window-bg); color: var(--win-text); padding: 8px; cursor: pointer; font-family: inherit; font-size: 1rem;">Easy</button>
    <button class="ms-diff-btn" data-diff="intermediate" style="width: 250px; text-align: center; border: 2px solid; border-color: var(--win-border-white) var(--win-border-dark) var(--win-border-dark) var(--win-border-white); background: var(--win-window-bg); color: var(--win-text); padding: 8px; cursor: pointer; font-family: inherit; font-size: 1rem;">Medium</button>
    <button class="ms-diff-btn" data-diff="expert" style="width: 250px; text-align: center; border: 2px solid; border-color: var(--win-border-white) var(--win-border-dark) var(--win-border-dark) var(--win-border-white); background: var(--win-window-bg); color: var(--win-text); padding: 8px; cursor: pointer; font-family: inherit; font-size: 1rem;">Hard</button>
  `;

  const btns = boardElement.querySelectorAll('.ms-diff-btn');
  btns.forEach(btn => {
    btn.addEventListener('click', (e) => {
      const diff = (e.currentTarget as HTMLButtonElement).getAttribute('data-diff') as any;
      currentMinesweeperDiff = diff;
      if (diff === 'expert') {
        engine = MinesweeperEngine.new_expert();
      } else if (diff === 'intermediate') {
        engine = MinesweeperEngine.new_intermediate();
      } else {
        engine = MinesweeperEngine.new_beginner();
      }

      renderBoard();
    });
    // Add pressed look
    btn.addEventListener('mousedown', (e) => {
      (e.currentTarget as HTMLElement).style.borderColor = 'var(--win-border-dark) var(--win-border-white) var(--win-border-white) var(--win-border-dark)';
    });
    btn.addEventListener('mouseup', (e) => {
      (e.currentTarget as HTMLElement).style.borderColor = 'var(--win-border-white) var(--win-border-dark) var(--win-border-dark) var(--win-border-white)';
    });
    btn.addEventListener('mouseleave', (e) => {
      (e.currentTarget as HTMLElement).style.borderColor = 'var(--win-border-white) var(--win-border-dark) var(--win-border-dark) var(--win-border-white)';
    });
  });
}

function showAIMenu(gameType: "chess" | "chess2" | "janggi") {
  boardElement.style.display = 'flex';
  boardElement.style.flexDirection = 'column';
  boardElement.style.justifyContent = 'center';
  boardElement.style.alignItems = 'center';
  boardElement.style.gap = '20px';
  boardElement.style.width = '400px';
  boardElement.style.height = '400px';
  boardElement.style.backgroundColor = 'var(--win-window-bg)';
  boardElement.style.border = '4px solid var(--win-border-light)';
  boardElement.style.boxShadow = 'inset 1px 1px var(--win-border-dark), inset -1px -1px var(--win-border-white)';

  const gameName = gameType === "chess" ? "Chess" : (gameType === "chess2" ? "Chess 2" : "Janggi");
  statusElement.textContent = `Start ${gameName}`;
  resetButton.style.display = 'none';

  boardElement.innerHTML = `
    <h3 style="margin: 0; color: var(--win-text);">Start ${gameName}</h3>
    <button class="ai-start-btn" data-ai="false" style="width: 250px; text-align: center; border: 2px solid; border-color: var(--win-border-white) var(--win-border-dark) var(--win-border-dark) var(--win-border-white); background: var(--win-window-bg); color: var(--win-text); padding: 8px; cursor: pointer; font-family: inherit; font-size: 1rem;">Player vs Player (Local)</button>
    <div style="width: 250px; height: 1px; background: var(--win-border-dark); margin: 5px 0;"></div>
    <button class="ai-start-btn" data-ai="true" data-depth="1" style="width: 250px; text-align: center; border: 2px solid; border-color: var(--win-border-white) var(--win-border-dark) var(--win-border-dark) var(--win-border-white); background: var(--win-window-bg); color: var(--win-text); padding: 8px; cursor: pointer; font-family: inherit; font-size: 1rem;">Play vs AI (Easy)</button>
    <button class="ai-start-btn" data-ai="true" data-depth="2" style="width: 250px; text-align: center; border: 2px solid; border-color: var(--win-border-white) var(--win-border-dark) var(--win-border-dark) var(--win-border-white); background: var(--win-window-bg); color: var(--win-text); padding: 8px; cursor: pointer; font-family: inherit; font-size: 1rem;">Play vs AI (Medium)</button>
    <button class="ai-start-btn" data-ai="true" data-depth="3" style="width: 250px; text-align: center; border: 2px solid; border-color: var(--win-border-white) var(--win-border-dark) var(--win-border-dark) var(--win-border-white); background: var(--win-window-bg); color: var(--win-text); padding: 8px; cursor: pointer; font-family: inherit; font-size: 1rem;">Play vs AI (Hard)</button>
  `;

  const btns = boardElement.querySelectorAll('.ai-start-btn');
  btns.forEach(btn => {
    btn.addEventListener('click', (e) => {
      const ai = (e.currentTarget as HTMLButtonElement).getAttribute('data-ai') === 'true';
      const depth = parseInt((e.currentTarget as HTMLButtonElement).getAttribute('data-depth') || '1');

      currentAiModeStatus = { enabled: ai, depth: depth };

      if (gameType === "chess") {
        engine = new ChessEngine();
      } else if (gameType === "chess2") {
        engine = new Chess2Engine();
      } else {
        engine = new JanggiEngine();
      }

      renderBoard();
    });
    // Add pressed look
    btn.addEventListener('mousedown', (e) => {
      (e.currentTarget as HTMLElement).style.borderColor = 'var(--win-border-dark) var(--win-border-white) var(--win-border-white) var(--win-border-dark)';
    });
    btn.addEventListener('mouseup', (e) => {
      (e.currentTarget as HTMLElement).style.borderColor = 'var(--win-border-white) var(--win-border-dark) var(--win-border-dark) var(--win-border-white)';
    });
    btn.addEventListener('mouseleave', (e) => {
      (e.currentTarget as HTMLElement).style.borderColor = 'var(--win-border-white) var(--win-border-dark) var(--win-border-dark) var(--win-border-white)';
    });
  });
}

async function loadGame(game: "chess" | "chess2" | "janggi" | "minesweeper" | "2048" | "racing" | "claw") {
  currentGame = game;
  selectedSquare = null;
  lastMove = null;
  validMovesForSelected = [];
  gameEndedRecorded = false;
  updateLeaderboardDisplay();

  stopRacingLoop(); // Clean up racing frame loop if it was active
  stopClawLoop();   // Clean up claw loop too

  if (game === "chess" || game === "chess2" || game === "janggi") {
    engine = null;
    showAIMenu(game);
    return;
  } else if (game === "minesweeper") {
    engine = null;
    showMinesweeperMenu();
    return; // Stop here, rendering triggers via the menu
  } else if (game === "2048") {
    engine = new Game2048Engine();
  } else if (game === "racing") {
    engine = new RacingGame();
    startRacingLoop();
    return; // Don't run standard renderBoard() which uses DOM grid
  } else if (game === "claw") {
    engine = new ClawGame();
    startClawLoop();
    return;
  }
  renderBoard();
}

menuButtons.forEach(btn => {
  btn.addEventListener('click', (e) => {
    // Remove active class from all
    menuButtons.forEach(b => b.classList.remove('active'));
    // Add to clicked
    const target = e.currentTarget as HTMLButtonElement;
    target.classList.add('active');

    const newGame = target.getAttribute('data-game') as "chess" | "chess2" | "janggi" | "minesweeper" | "2048" | "racing" | "claw";

    if (newGame) {
      loadGame(newGame);
    }
  });
});

async function start() {
  await initChess();
  await initChess2();
  await initJanggi();
  await initMinesweeper();
  await init2048();
  await initRacing();
  await initClaw();
  loadGame("chess");
}

start();
