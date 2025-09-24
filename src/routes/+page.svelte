<script lang="ts">
  let selectedBreak: number = 5; // Default to 5 minutes
  let timerMinutes = 20; // Default timer duration
  let isRunning = false;
  let currentTime = timerMinutes * 60; // Current time in seconds
  let interval: number | null = null;
  let isBreakTime = false;

  // Reactive statements
  $: sessionTime = timerMinutes + Number(selectedBreak);
  $: displayMinutes = Math.floor(currentTime / 60);
  $: displaySeconds = currentTime % 60;

  function incrementTimer() {
    // Increase in 5 minute steps
    timerMinutes = Math.min(timerMinutes + 5, 99);
    if (!isRunning) currentTime = timerMinutes * 60;
  }

  function decrementTimer() {
    // Decrease in 5 minute steps, with a minimum of 5 minutes
    timerMinutes = Math.max(timerMinutes - 5, 5);
    if (!isRunning) currentTime = timerMinutes * 60;
  }

  function startStop() {
    if (isRunning) {
      // Stop the timer
      if (interval) {
        clearInterval(interval);
        interval = null;
      }
      isRunning = false;
    } else {
      // Start the timer
      isRunning = true;
      interval = setInterval(() => {
        currentTime--;
        
        if (currentTime <= 0) {
          // Timer finished
          if (isBreakTime) {
            // Break finished, start work session
            isBreakTime = false;
            currentTime = timerMinutes * 60;
          } else {
            // Work session finished, start break
            isBreakTime = true;
            currentTime = selectedBreak * 60;
          }
        }
      }, 1000);
    }
  }

  function reset() {
    if (interval) {
      clearInterval(interval);
      interval = null;
    }
    isRunning = false;
    isBreakTime = false;
    currentTime = timerMinutes * 60;
  }
</script>

<main class="container">
  <div class="timer-container">
    <div class="time-controls">
      <div class="time minus">
        <button on:click={decrementTimer} disabled={isRunning}>-</button>
      </div>
      <div class="time display">
        {String(displayMinutes).padStart(2, '0')}:{String(displaySeconds).padStart(2, '0')}
      </div>
      <div class="time plus">
        <button on:click={incrementTimer} disabled={isRunning}>+</button>
      </div>
    </div>
    <div class="break-selector" class:disabled={isRunning}>
      <div class="break-option">
        <input type="radio" id="break-5" name="break" value={5} bind:group={selectedBreak} disabled={isRunning} />
        <label for="break-5">5</label>
      </div>
      <div class="break-option">
        <input type="radio" id="break-10" name="break" value={10} bind:group={selectedBreak} disabled={isRunning} />
        <label for="break-10">10</label>
      </div>
      <div class="break-option">
        <input type="radio" id="break-15" name="break" value={15} bind:group={selectedBreak} disabled={isRunning} />
        <label for="break-15">15</label>
      </div>
    </div>
  </div>

  <div class="session-controls">
    <div class="session-info">
      <div class="session-time">
        <span class="time-value">{sessionTime} min</span>
      </div>
      <div class="session-phase">
        <span class="phase-indicator" class:work={!isBreakTime} class:break={isBreakTime}>
          {isBreakTime ? 'Break Time' : 'Work Time'}
        </span>
      </div>
    </div>

    <div class="control-buttons">
      <button class="start-stop" class:running={isRunning} on:click={startStop}>
        {isRunning ? 'Stop' : 'Start'}
      </button>
      <button class="reset" on:click={reset} disabled={isRunning}>
        Reset
      </button>
    </div>
  </div>
</main>

<style>
.container {
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;
}

.timer-container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  margin-top: 2rem;
}

.time-controls {
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  gap: 20px;
  margin-bottom: 2rem;
}

.time {
  font-size: 3rem;
  font-weight: 700;
}

.time button {
  color: inherit;
  font-size: 2rem;
  background: none;
  border: none;
  cursor: pointer;
  transition: opacity 0.2s ease;
}

.time button:disabled {
  opacity: 0.3;
  pointer-events: none;
}

.break-selector {
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  gap: 15px;
  transition: opacity 0.2s ease;
}

.break-selector.disabled {
  opacity: 0.3;
  pointer-events: none;
}

.break-option {
  position: relative;
}

.break-option input[type="radio"] {
  opacity: 0;
  position: absolute;
  width: 0;
  height: 0;
}

.break-option label {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background-color: transparent;
  border: 2px solid var(--color-text);
  cursor: pointer;
  font-size: 1.1rem;
  font-weight: 500;
  transition: all 0.2s ease;
}

.break-option input[type="radio"]:disabled + label {
  pointer-events: none;
}

.break-option input[type="radio"]:checked + label {
  background-color: var(--color-fg);
  border-color: var(--color-fg);
  color: white;
}

.break-option label:hover {
  border-color: var(--color-fg);
  color: var(--color-fg);
}

.break-option input[type="radio"]:checked + label:hover {
  color: white;
}

.session-controls {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2rem;
  margin-top: 2rem;
}

.session-info {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
}

.session-time {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
}

.session-time .time-value {
  font-size: 1.2rem;
  font-weight: 700;
  color: var(--color-fg);
}

.session-phase {
  margin: 0.5rem 0;
}

.phase-indicator {
  padding: 0.5rem 1rem;
  border-radius: 20px;
  font-size: 0.9rem;
  font-weight: 600;
  transition: all 0.3s ease;
}

.phase-indicator.work {
  background-color: rgba(188, 142, 197, 0.2);
  color: var(--color-fg);
  border: 1px solid var(--color-fg);
}

.phase-indicator.break {
  background-color: rgba(76, 175, 80, 0.2);
  color: #4caf50;
  border: 1px solid #4caf50;
}

.control-buttons {
  display: flex;
  gap: 1.5rem;
  align-items: center;
}

.control-buttons button {
  padding: 0.8rem 2rem;
  font-size: 1.1rem;
  font-weight: 600;
  border-radius: 25px;
  border: 2px solid;
  cursor: pointer;
  transition: all 0.3s ease;
  font-family: inherit;
  min-width: 100px;
}

.start-stop {
  background-color: var(--color-fg);
  color: white;
  border-color: var(--color-fg);
}

.start-stop:hover {
  background-color: transparent;
  color: var(--color-fg);
}

.start-stop.running {
  background-color: #f44336;
  border-color: #f44336;
}

.start-stop.running:hover {
  background-color: transparent;
  color: #f44336;
}

.reset {
  background-color: transparent;
  color: var(--color-text);
  border-color: var(--color-text);
}

.reset:hover {
  background-color: transparent;
  border-color: var(--color-fg);
  color: var(--color-fg);
}

.reset:disabled {
  opacity: 0.3;
  pointer-events: none;
}

.reset:disabled:hover {
  background-color: transparent;
  color: var(--color-text);
}
</style>
