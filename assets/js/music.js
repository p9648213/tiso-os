let musicCleanUpEvent = [];

export function setupMusicPlayer() {
  const audio = new Audio();
  let currentSongIndex = 0;
  let playlist = [];
  let isPlaying = false;
  let isShuffle = false;
  let repeatMode = "none";
  let audioContext;
  let analyser;
  let source;
  let animationId;
  let canvas;
  let canvasContext;

  const playPauseBtn = document.getElementById("play-pause-btn");
  const playIcon = document.getElementById("play-icon");
  const pauseIcon = document.getElementById("pause-icon");
  const prevBtn = document.getElementById("prev-btn");
  const nextBtn = document.getElementById("next-btn");
  const seekBar = document.getElementById("seek-bar");
  const progressBar = document.getElementById("progress-bar");
  const currentTimeEl = document.getElementById("current-time");
  const totalTimeEl = document.getElementById("total-time");
  const volumeSlider = document.getElementById("volume-slider");
  const shuffleBtn = document.getElementById("shuffle-btn");
  const repeatBtn = document.getElementById("repeat-btn");
  const loadMusicBtn = document.getElementById("load-music-btn");
  const musicFileInput = document.getElementById("music-file-input");
  const playlistContainer = document.getElementById("playlist-container");
  const songTitle = document.getElementById("song-title");
  const artistName = document.getElementById("artist-name");
  const albumArtContainer = document.getElementById("album-art");

  audio.volume = 0.7;

  function initAudioContext() {
    if (!audioContext) {
      const AudioContext = window.AudioContext || window.webkitAudioContext;
      if (AudioContext) {
        audioContext = new AudioContext();
        analyser = audioContext.createAnalyser();
        analyser.fftSize = 256;
        createVisualizationCanvas();
      } else {
        console.warn("Web Audio API is not supported in this browser");
      }
    }
  }

  function createVisualizationCanvas() {
    albumArtContainer.innerHTML = "";
    canvas = document.createElement("canvas");
    canvas.width = 128;
    canvas.height = 128;
    canvas.className = "w-32 h-32 rounded-sm";
    albumArtContainer.appendChild(canvas);
    canvasContext = canvas.getContext("2d");
    canvasContext.fillStyle = "#1e293b";
    canvasContext.fillRect(0, 0, canvas.width, canvas.height);
  }

  playPauseBtn.addEventListener("click", togglePlayPause);
  prevBtn.addEventListener("click", playPreviousSong);
  nextBtn.addEventListener("click", playNextSong);
  seekBar.addEventListener("input", seekAudio);
  volumeSlider.addEventListener("input", changeVolume);
  shuffleBtn.addEventListener("click", toggleShuffle);
  repeatBtn.addEventListener("click", toggleRepeat);
  loadMusicBtn.addEventListener("click", () => musicFileInput.click());
  musicFileInput.addEventListener("change", loadMusicFiles);
  audio.addEventListener("timeupdate", updateProgress);
  audio.addEventListener("loadedmetadata", updateDuration);
  audio.addEventListener("ended", handleSongEnd);
  audio.addEventListener("play", () => {
    initAudioContext();
    if (audioContext && analyser) {
      if (!source) {
        source = audioContext.createMediaElementSource(audio);
        source.connect(analyser);
        analyser.connect(audioContext.destination);
      }
      visualize();
    }
  });
  audio.addEventListener("pause", () => {
    if (animationId) {
      cancelAnimationFrame(animationId);
    }
  });

  function togglePlayPause() {
    if (playlist.length === 0) return;

    if (isPlaying) {
      audio.pause();
      playIcon.classList.remove("hidden");
      pauseIcon.classList.add("hidden");
    } else {
      audio.play();
      playIcon.classList.add("hidden");
      pauseIcon.classList.remove("hidden");
    }
    isPlaying = !isPlaying;
  }

  function playPreviousSong() {
    if (playlist.length === 0) return;

    if (isShuffle) {
      currentSongIndex = Math.floor(Math.random() * playlist.length);
    } else {
      currentSongIndex =
        (currentSongIndex - 1 + playlist.length) % playlist.length;
    }
    loadSong(currentSongIndex);
    if (isPlaying) audio.play();
  }

  function playNextSong() {
    if (playlist.length === 0) return;

    if (isShuffle) {
      currentSongIndex = Math.floor(Math.random() * playlist.length);
    } else {
      currentSongIndex = (currentSongIndex + 1) % playlist.length;
    }
    loadSong(currentSongIndex);
    if (isPlaying) audio.play();
  }

  function seekAudio() {
    const seekTime = (seekBar.value / 100) * audio.duration;
    audio.currentTime = seekTime;
  }

  function changeVolume() {
    audio.volume = volumeSlider.value / 100;
  }

  function toggleShuffle() {
    isShuffle = !isShuffle;
    shuffleBtn.querySelector("img").src = isShuffle
      ? "/assets/images/shuffle-on.svg"
      : "/assets/images/shuffle-off.svg";
  }

  function toggleRepeat() {
    if (repeatMode === "none") {
      repeatMode = "one";
      repeatBtn.querySelector("img").src = "/assets/images/repeat-one.svg";
    } else if (repeatMode === "one") {
      repeatMode = "all";
      repeatBtn.querySelector("img").src = "/assets/images/repeat.svg";
    } else {
      repeatMode = "none";
      repeatBtn.querySelector("img").src = "/assets/images/repeat-off.svg";
    }
  }

  function loadMusicFiles() {
    const files = Array.from(musicFileInput.files);

    if (files.length === 0) return;

    if (playlist.length === 0) {
      playlistContainer.innerHTML = "";
    }

    files.forEach((file) => {
      const song = {
        file: file,
        name: file.name.replace(/\.[^/.]+$/, ""),
        url: URL.createObjectURL(file),
      };
      playlist.push(song);
      addSongToPlaylist(song, playlist.length - 1);
    });

    if (playlist.length === files.length) {
      currentSongIndex = 0;
      loadSong(currentSongIndex);
    }
  }

  function addSongToPlaylist(song, index) {
    const songElement = document.createElement("div");
    songElement.className =
      "flex items-center gap-3 p-2 rounded-sm hover:bg-zinc-700";
    songElement.dataset.index = index;

    songElement.innerHTML = `
            <div class="flex-1">
                <div class="text-sm font-medium">${song.name}</div>
            </div>
        `;

    songElement.addEventListener("click", () => {
      currentSongIndex = index;
      loadSong(currentSongIndex);
      if (!isPlaying) togglePlayPause();
    });

    playlistContainer.appendChild(songElement);
  }

  function loadSong(index) {
    if (index < 0 || index >= playlist.length) return;

    const song = playlist[index];
    audio.src = song.url;
    songTitle.textContent = song.name;
    artistName.textContent = "Local file";

    if (canvas && canvasContext) {
      canvasContext.fillStyle = "#1e293b";
      canvasContext.fillRect(0, 0, canvas.width, canvas.height);
    }

    document.querySelectorAll("#playlist-container > div").forEach((el, i) => {
      if (i === index) {
        el.classList.add("bg-zinc-700");
      } else {
        el.classList.remove("bg-zinc-700");
      }
    });
  }

  function updateProgress() {
    if (audio.duration) {
      const progressPercent = (audio.currentTime / audio.duration) * 100;
      progressBar.style.width = `${progressPercent}%`;
      seekBar.value = progressPercent;

      const currentMinutes = Math.floor(audio.currentTime / 60);
      const currentSeconds = Math.floor(audio.currentTime % 60);
      currentTimeEl.textContent = `${currentMinutes}:${currentSeconds
        .toString()
        .padStart(2, "0")}`;
    }
  }

  function updateDuration() {
    const durationMinutes = Math.floor(audio.duration / 60);
    const durationSeconds = Math.floor(audio.duration % 60);
    totalTimeEl.textContent = `${durationMinutes}:${durationSeconds
      .toString()
      .padStart(2, "0")}`;
  }

  function handleSongEnd() {
    if (repeatMode === "one") {
      audio.currentTime = 0;
      audio.play();
    } else if (repeatMode === "all" || currentSongIndex < playlist.length - 1) {
      playNextSong();
    } else {
      isPlaying = false;
      playIcon.classList.remove("hidden");
      pauseIcon.classList.add("hidden");
    }
  }

  function visualize() {
    if (!analyser || !canvas || !canvasContext) return;

    const bufferLength = analyser.frequencyBinCount;
    const dataArray = new Uint8Array(bufferLength);

    function draw() {
      animationId = requestAnimationFrame(draw);

      analyser.getByteFrequencyData(dataArray);

      canvasContext.fillStyle = "#1e293b";
      canvasContext.fillRect(0, 0, canvas.width, canvas.height);

      const barWidth = (canvas.width / bufferLength) * 2.5;
      let barHeight;
      let x = 0;

      const gradient = canvasContext.createLinearGradient(
        0,
        canvas.height,
        0,
        0
      );
      gradient.addColorStop(0, "#3b82f6");
      gradient.addColorStop(0.5, "#8b5cf6");
      gradient.addColorStop(1, "#ec4899");

      for (let i = 0; i < bufferLength; i++) {
        barHeight = (dataArray[i] / 255) * canvas.height * 0.8;

        canvasContext.fillStyle = gradient;
        canvasContext.fillRect(
          x,
          canvas.height - barHeight,
          barWidth,
          barHeight
        );

        canvasContext.fillStyle = `rgba(59, 130, 246, 0.2)`;
        canvasContext.fillRect(x, canvas.height - barHeight - 5, barWidth, 3);

        x += barWidth + 1;
      }

      canvasContext.shadowBlur = 10;
      canvasContext.shadowColor = "#3b82f6";

      canvasContext.strokeStyle = "rgba(255, 255, 255, 0.2)";
      canvasContext.lineWidth = 1;
      canvasContext.beginPath();
      canvasContext.moveTo(0, canvas.height / 2);
      canvasContext.lineTo(canvas.width, canvas.height / 2);
      canvasContext.stroke();

      canvasContext.shadowBlur = 0;
    }

    draw();
  }

  const closeBtn = document.querySelector("#music-player-header .close");
  closeBtn.addEventListener("click", function () {
    if (audioContext) {
      audioContext.close();
    }

    if (animationId) {
      cancelAnimationFrame(animationId);
    }

    document.getElementById("music-player-window").remove();
    document.getElementById("taskbar-music-player-window").remove();
    musicCleanUpEvent.forEach((event) => {
      document.removeEventListener(event.event, event.handler);
    });
    musicCleanUpEvent = [];
  });
}

export function centerMusicWindow() {
  const win = document.getElementById("music-player-window");
  const rect = win.getBoundingClientRect();
  const top = (window.innerHeight - rect.height) / 2;
  const left = (window.innerWidth - rect.width) / 2;
  win.style.top = `${top - 48}px`;
  win.style.left = `${left}px`;
}

export function setupMusicWindowGrab() {
  const musicHeader = document.getElementById(`music-player-header`);
  const musicWindow = document.getElementById(`music-player-window`);

  let isDragging = false;
  let offsetX = 0;
  let offsetY = 0;

  musicHeader.addEventListener("mousedown", (event) => {
    isDragging = true;
    const rect = musicWindow.getBoundingClientRect();
    offsetX = event.clientX - rect.left;
    offsetY = event.clientY - rect.top;

    event.preventDefault();
  });

  function handleMouseMove(event) {
    if (!isDragging) return;

    musicWindow.style.left = `${event.clientX - offsetX}px`;
    musicWindow.style.top = `${event.clientY - offsetY}px`;
  }

  function handleMouseUp() {
    isDragging = false;
  }

  musicCleanUpEvent.push({
    event: "mousemove",
    handler: handleMouseMove,
  });
  musicCleanUpEvent.push({ event: "mouseup", handler: handleMouseUp });

  document.addEventListener("mousemove", handleMouseMove);
  document.addEventListener("mouseup", handleMouseUp);
}
