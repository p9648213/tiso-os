export function setupGlobalVariables() {
  window.selectedItem = null;
  window.editMode = false;
  window.canvasRunning = [];
  window.snakeState = {
    wasmModule: null,
    canvasContainer: null,
  };
  window.flappyBirdState = {
    wasmModule: null,
    canvasContainer: null,
  };
}
