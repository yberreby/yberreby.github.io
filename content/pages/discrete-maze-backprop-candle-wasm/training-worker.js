// Training worker for maze backpropagation
// Loads WASM module and runs training in background

let wasm = null;
let TrainingSession = null;
let session = null;
let currentState = null;
let isTraining = false;
let maxSteps = 1000;
let convergenceThreshold = 0.99;

// Initialize WASM module
async function initWasm() {
  const startTime = performance.now();
  postMessage({
    type: "timing",
    message: `Starting WASM initialization at ${startTime.toFixed(2)}ms`,
  });

  try {
    const importStart = performance.now();
    const wasmModule = await import("./code/pkg/diffmaze.js");
    postMessage({
      type: "timing",
      message: `JS module imported in ${(performance.now() - importStart).toFixed(2)}ms`,
    });

    const fetchStart = performance.now();
    const wasmResponse = await fetch("./code/pkg/diffmaze_bg.wasm");
    const wasmBuffer = await wasmResponse.arrayBuffer();
    postMessage({
      type: "timing",
      message: `WASM file fetched in ${(performance.now() - fetchStart).toFixed(2)}ms, size: ${(wasmBuffer.byteLength / 1024).toFixed(2)}KB`,
    });

    const initStart = performance.now();

    // Pre-allocate memory - 64MB initial, max 256MB
    const memory = new WebAssembly.Memory({
      initial: 1024, // 64MB (1024 * 64KB pages)
      maximum: 4096, // 256MB
    });

    postMessage({
      type: "timing",
      message: `Memory allocated: ${memory.buffer.byteLength / (1024 * 1024)}MB`,
    });

    // Initialize with pre-allocated memory
    await wasmModule.default({
      module: wasmBuffer,
      memory: memory,
    });
    TrainingSession = wasmModule.TrainingSession;
    postMessage({
      type: "timing",
      message: `WASM module initialized in ${(performance.now() - initStart).toFixed(2)}ms`,
    });

    postMessage({
      type: "timing",
      message: `Total WASM init time: ${(performance.now() - startTime).toFixed(2)}ms`,
    });
    postMessage({ type: "wasm-loaded" });
  } catch (error) {
    postMessage({
      type: "error",
      message: "Failed to load WASM: " + error.message,
    });
  }
}

// Training loop that runs continuously
function trainLoop() {
  if (!session || !isTraining) return;

  const currentStep = session.get_step();
  if (currentStep >= maxSteps) {
    isTraining = false;
    if (currentState) {
      currentState.isComplete = true;
      currentState.reason = "max-steps";
    }
    return;
  }

  // Perform one training step
  const stepData = session.step();
  if (stepData.length > 0) {
    // Extract probabilities and loss
    const probs = stepData.slice(0, -1);
    const loss = stepData[stepData.length - 1];

    // Get logits
    const logits = session.get_logits();

    // Update current state
    currentState = {
      currentStep: session.get_step(),
      probs: probs,
      loss: loss,
      logits: logits,
      isComplete: false,
    };

    // Check for convergence
    if (-loss > convergenceThreshold) {
      isTraining = false;
      currentState.isComplete = true;
      currentState.reason = "converged";
      return;
    }
  }

  // Continue training on next tick
  if (isTraining) {
    setTimeout(trainLoop, 0);
  }
}

// Message handler
onmessage = async (e) => {
  switch (e.data.type) {
    case "init-wasm":
      await initWasm();
      break;

    case "init-session":
      const { width, height, wallProb, learningRate } = e.data;
      const sessionStart = performance.now();

      session = TrainingSession.new(width, height, wallProb, learningRate);

      if (session) {
        postMessage({
          type: "timing",
          message: `Training session created in ${(performance.now() - sessionStart).toFixed(2)}ms`,
        });

        // Get maze info
        const mazeInfoStart = performance.now();
        const mazeInfo = session.get_maze_info();
        postMessage({
          type: "timing",
          message: `Maze info retrieved in ${(performance.now() - mazeInfoStart).toFixed(2)}ms`,
        });

        postMessage({
          type: "session-initialized",
          mazeInfo: Array.from(mazeInfo),
        });

        // Reset state
        currentState = null;
        isTraining = false;
      } else {
        postMessage({
          type: "error",
          message: "Failed to create training session",
        });
      }
      break;

    case "start-training":
      if (!session) {
        postMessage({ type: "error", message: "No session initialized" });
        return;
      }

      maxSteps = e.data.maxSteps;
      convergenceThreshold = e.data.convergenceThreshold;
      isTraining = true;
      trainLoop();
      break;

    case "stop-training":
      isTraining = false;
      break;

    case "get-state":
      if (currentState) {
        postMessage({
          type: "state",
          ...currentState,
        });
      } else {
        postMessage({
          type: "state",
          currentStep: 0,
          probs: null,
          loss: 0,
          isComplete: false,
        });
      }
      break;
  }
};
