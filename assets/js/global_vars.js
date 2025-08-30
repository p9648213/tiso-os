export function setupGlobalVariables() {
  window.selectedItem = null;
  window.editMode = false;
  window.snakeState = {
    isRunning: false,
    wasmModule: null,
    canvasContainer: null,
  };
  window.flappyBirdState = {
    isRunning: false,
    wasmModule: null,
    canvasContainer: null,
  };
}
