export function setupCalculatorWindow() {
  const display = document.getElementById("calculator-display");
  const buttonsContainer = document.getElementById("calculator-buttons");

  let currentValue = "0";
  let previousValue = null;
  let operator = null;
  let shouldResetDisplay = false;

  function updateDisplay() {
    display.textContent = currentValue;
  }

  function handleNumber(value) {
    if (shouldResetDisplay) {
      currentValue = value;
      shouldResetDisplay = false;
    } else {
      if (currentValue === "0" && value !== ".") {
        currentValue = value;
      } else {
        if (value === "." && currentValue.includes(".")) return;
        currentValue += value;
      }
    }
    updateDisplay();
  }

  function handleOperator(op) {
    if (operator !== null && !shouldResetDisplay) {
      handleEquals();
    }
    previousValue = currentValue;
    operator = op;
    shouldResetDisplay = true;
  }

  function handleEquals() {
    if (operator === null || previousValue === null) return;

    const prev = parseFloat(previousValue);
    const current = parseFloat(currentValue);

    let result;
    switch (operator) {
      case "+":
        result = prev + current;
        break;
      case "-":
        result = prev - current;
        break;
      case "*":
        result = prev * current;
        break;
      case "/":
        if (current === 0) {
          currentValue = "Error";
          updateDisplay();
          setTimeout(() => {
            handleClear();
          }, 1500);
          return;
        }
        result = prev / current;
        break;
      default:
        return;
    }

    currentValue = String(
      Math.round((result + Number.EPSILON) * 100000000) / 100000000
    );
    operator = null;
    previousValue = null;
    shouldResetDisplay = true;
    updateDisplay();
  }

  function handleClear() {
    currentValue = "0";
    previousValue = null;
    operator = null;
    shouldResetDisplay = false;
    updateDisplay();
  }

  function handleDelete() {
    if (currentValue.length > 1) {
      currentValue = currentValue.slice(0, -1);
    } else {
      currentValue = "0";
    }
    updateDisplay();
  }

  buttonsContainer.addEventListener("click", (event) => {
    const button = event.target.closest("button");
    if (!button) return;

    const action = button.dataset.action;
    const value = button.dataset.value;

    switch (action) {
      case "number":
        handleNumber(value);
        break;
      case "operator":
        handleOperator(value);
        break;
      case "equals":
        handleEquals();
        break;
      case "clear":
        handleClear();
        break;
      case "delete":
        handleDelete();
        break;
    }
  });

  document.addEventListener("keydown", (event) => {
    const calculatorWindow = document.getElementById("calculator-window");
    if (!calculatorWindow) return;

    const key = event.key;

    if (key >= "0" && key <= "9") {
      handleNumber(key);
    } else if (key === ".") {
      handleNumber(key);
    } else if (key === "+" || key === "-" || key === "*" || key === "/") {
      handleOperator(key);
    } else if (key === "Enter" || key === "=") {
      event.preventDefault();
      handleEquals();
    } else if (key === "Escape" || key === "c" || key === "C") {
      handleClear();
    } else if (key === "Backspace") {
      event.preventDefault();
      handleDelete();
    }
  });
}

export function setupCalculatorToolBar() {
  const calculatorToolBar = document.getElementById("calculator-header");

  const close = calculatorToolBar.querySelector(".close");

  close.addEventListener("click", function () {
    document.getElementById("calculator-window").remove();
    document.getElementById("taskbar-calculator-window").remove();
  });
}
