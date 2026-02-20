# WebAssembly(Wasm) 로컬 개발 및 테스트 가이드

웹 개발자에게 익숙한 `npm run dev` 환경(HMR 포함)에서 Rust WebAssembly 모듈을 개발하고 테스트하는 방법입니다. 현재 사용 중이신 **Vite** 환경(`chess-web`)에 최적화된 워크플로우를 안내합니다.

## 1. 프로젝트 구조 이해하기

추천하는 기본 폴더 구조는 프론트엔드 코드(TS/JS)와 Wasm 코드(Rust)를 분리하는 것입니다.

```text
최상위 폴더 (예: g1/)
 ├── chess-web/        # 프론트엔드 (Vite + TypeScript) - 현재 계신 곳!
 │    ├── package.json
 │    └── src/
 │
 └── chess-wasm/       # WebAssembly 모듈 (Rust)
      ├── Cargo.toml
      └── src/
```

> **주의:** 아까 실행하신 명령어에서 에러가 났던 이유는 `chess-wasm` 모듈(Rust 프로젝트)이 아직 생성되지 않았거나, 해당 폴더 안에서 `wasm-pack build`를 실행하지 않았기 때문입니다.

## 2. Rust(Wasm) 프로젝트 생성 및 빌드

먼저 Rust Wasm 모듈을 작성할 프로젝트를 생성합니다.

```bash
# 최상위 폴더(g1/)에서 실행
cargo new --lib chess-wasm
cd chess-wasm

# Cargo.toml에 wasm-bindgen 의존성 추가 설정 (아래 3번 참고)
# 설정 후 빌드 진행 (target: web)
wasm-pack build --target web
```
*`--target web`으로 빌드해야 브라우저에서 네이티브 ES Module로 직접 Wasm을 불러올 수 있습니다.*

## 3. 프론트엔드(`chess-web`)와 연결하기

Rust 프로젝트가 성공적으로 빌드되면 `chess-wasm/pkg` 폴더가 생성됩니다. 이를 `chess-web` 패키지에 로컬 모듈로 설치합니다.

```bash
# chess-web 폴더로 이동
cd ../chess-web

# 빌드된 로컬 wasm 패키지 설치
npm install ../chess-wasm/pkg
```

## 4. 로컬 서버 실행 및 개발 (`npm run dev`)

이제 웹 개발 환경에서 익숙한 방식 그대로 로컬 서버를 띄울 수 있습니다.

```bash
# chess-web 폴더 안에서 실행
npm run dev
```

서버가 띄워지면, `main.ts` 파일에서 다음과 같이 Wasm 함수를 불러와 사용할 수 있습니다.

```typescript
import init, { greet } from 'chess-wasm'; 
// greet는 Rust 쪽에서 작성한 테스트용 함수 (예시)

async function runWasm() {
  // 1. 브라우저 환경(target web)에서는 반드시 init()을 먼저 호출해 Wasm 메모리를 초기화해야 합니다.
  await init();
  
  // 2. 이후 Rust 함수를 자유롭게 호출합니다.
  greet();
}

runWasm();
```

## 5. 전체 개발 사이클 (수정 후 반영 과정)

Wasm 코드를 수정했을 때의 작업 흐름은 다음과 같습니다.

1. **Rust 코드 수정:** `chess-wasm/src/lib.rs` 등에 원하는 체스 로직 수정
2. **Wasm 재빌드:** `chess-wasm` 폴더에서 `wasm-pack build --target web` 실행
3. **변경사항 확인:** `chess-web`에서 돌고 있는 `npm run dev` 서버가 파일 변경을 감지하고 프론트엔드 페이지를 자동으로 새로고침(HMR)하여 즉시 반영결과를 보여줍니다.

> **Tip:** 매번 수동으로 `wasm-pack build`를 치는 것이 번거롭다면, `chess-web/package.json`의 `scripts` 영역에 빌드 명령어를 매크로처럼 묶어서(예: `"dev": "cd ../chess-wasm && wasm-pack build --target web && cd ../chess-web && vite"`) 사용하는 방법도 있습니다.
