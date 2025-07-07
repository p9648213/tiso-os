let clockIntervalId = null;

export function setupClock() {
  const timeEl = document.getElementById("clock-time");
  const dateEl = document.getElementById("clock-date");

  if (!timeEl || !dateEl) {
    console.error("Clock elements not found!");
    return;
  }

  if (clockIntervalId) {
    clearInterval(clockIntervalId);
  }

  updateClock(timeEl, dateEl);

  clockIntervalId = setInterval(() => updateClock(timeEl, dateEl), 1000);
}

function updateClock(timeEl, dateEl) {
  const now = new Date();

  const timeOptions = {
    hour: "numeric",
    minute: "2-digit",
    hour12: true,
  };
  const timeString = new Intl.DateTimeFormat("en-US", timeOptions).format(now);

  const dateOptions = {
    year: "numeric",
    month: "numeric",
    day: "numeric",
  };
  const dateString = new Intl.DateTimeFormat("en-US", dateOptions).format(now);

  timeEl.textContent = timeString;
  dateEl.textContent = dateString;
}
