// assets/js/music.js
export function setupMusicPlayer() {
  const audio = new Audio();
  let currentSongIndex = 0;
  let playlist = [];
  let isPlaying = false;
  let isShuffle = false;
  let repeatMode = "none"; // 'none', 'one', 'all'
  let audioContext;
  let analyser;
  let source;
  let animationId;
  let canvas;
  let canvasContext;

  // DOM elements
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

  // Set initial volume
  audio.volume = 0.7;

  // Initialize audio context and analyser
  function initAudioContext() {
    if (!audioContext) {
      // Properly check for AudioContext support with fallback to prefixed version
      const AudioContext = window.AudioContext || window.webkitAudioContext;
      if (AudioContext) {
        audioContext = new AudioContext();
        analyser = audioContext.createAnalyser();
        analyser.fftSize = 256;

        // Create canvas for visualization
        createVisualizationCanvas();
      } else {
        console.warn("Web Audio API is not supported in this browser");
        // Keep the default album art
      }
    }
  }

  // Create canvas for wave visualization
  function createVisualizationCanvas() {
    // Clear the container and create a canvas element
    albumArtContainer.innerHTML = "";
    canvas = document.createElement("canvas");
    canvas.width = 128;
    canvas.height = 128;
    canvas.className = "w-32 h-32 rounded-sm";
    albumArtContainer.appendChild(canvas);
    canvasContext = canvas.getContext("2d");

    // Initialize with a dark background
    canvasContext.fillStyle = "#1e293b";
    canvasContext.fillRect(0, 0, canvas.width, canvas.height);
  }

  // Event listeners
  playPauseBtn.addEventListener("click", togglePlayPause);
  prevBtn.addEventListener("click", playPreviousSong);
  nextBtn.addEventListener("click", playNextSong);
  seekBar.addEventListener("input", seekAudio);
  volumeSlider.addEventListener("input", changeVolume);
  shuffleBtn.addEventListener("click", toggleShuffle);
  repeatBtn.addEventListener("click", toggleRepeat);
  loadMusicBtn.addEventListener("click", () => musicFileInput.click());
  musicFileInput.addEventListener("change", loadMusicFiles);

  // Audio event listeners
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

  // Functions
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

    // Clear existing playlist if this is a new load
    if (playlist.length === 0) {
      playlistContainer.innerHTML = "";
    }

    // Add files to playlist
    files.forEach((file) => {
      const song = {
        file: file,
        name: file.name.replace(/\.[^/.]+$/, ""), // Remove file extension
        url: URL.createObjectURL(file),
      };
      playlist.push(song);
      addSongToPlaylist(song, playlist.length - 1);
    });

    // If this is the first song loaded, play it
    if (playlist.length === files.length) {
      currentSongIndex = 0;
      loadSong(currentSongIndex);
    }
  }

  function addSongToPlaylist(song, index) {
    const songElement = document.createElement("div");
    songElement.className =
      "flex items-center gap-3 p-2 rounded-sm hover:bg-zinc-700 cursor-pointer";
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

    // Reset visualization if canvas exists
    if (canvas && canvasContext) {
      canvasContext.fillStyle = "#1e293b";
      canvasContext.fillRect(0, 0, canvas.width, canvas.height);
    }

    // Update active song in playlist
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

      // Update current time
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
      // End of playlist
      isPlaying = false;
      playIcon.classList.remove("hidden");
      pauseIcon.classList.add("hidden");
    }
  }

  // Visualize audio data with wave effect
  function visualize() {
    if (!analyser || !canvas || !canvasContext) return;

    const bufferLength = analyser.frequencyBinCount;
    const dataArray = new Uint8Array(bufferLength);

    function draw() {
      animationId = requestAnimationFrame(draw);

      analyser.getByteFrequencyData(dataArray);

      // Clear canvas
      canvasContext.fillStyle = "#1e293b";
      canvasContext.fillRect(0, 0, canvas.width, canvas.height);

      // Draw wave
      const barWidth = (canvas.width / bufferLength) * 2.5;
      let barHeight;
      let x = 0;

      // Create gradient for the bars
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

        // Draw the bar
        canvasContext.fillStyle = gradient;
        canvasContext.fillRect(
          x,
          canvas.height - barHeight,
          barWidth,
          barHeight
        );

        // Add a reflection effect
        canvasContext.fillStyle = `rgba(59, 130, 246, 0.2)`;
        canvasContext.fillRect(x, canvas.height - barHeight - 5, barWidth, 3);

        x += barWidth + 1;
      }

      // Add a subtle glow effect
      canvasContext.shadowBlur = 10;
      canvasContext.shadowColor = "#3b82f6";

      // Draw center line
      canvasContext.strokeStyle = "rgba(255, 255, 255, 0.2)";
      canvasContext.lineWidth = 1;
      canvasContext.beginPath();
      canvasContext.moveTo(0, canvas.height / 2);
      canvasContext.lineTo(canvas.width, canvas.height / 2);
      canvasContext.stroke();

      // Reset shadow
      canvasContext.shadowBlur = 0;
    }

    draw();
  }

  // Close button functionality
  const closeBtn = document.querySelector("#music-player-header .close");
  closeBtn.addEventListener("click", function () {
    // Clean up audio context
    if (audioContext) {
      audioContext.close();
    }

    // Cancel any animation frames
    if (animationId) {
      cancelAnimationFrame(animationId);
    }

    document.getElementById("music-player-window").remove();
    document.getElementById("taskbar-music-player-window").remove();
  });
}
