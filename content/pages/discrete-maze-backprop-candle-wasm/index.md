+++
title = "Backpropagating through a maze with candle and WASM"
path = "discrete-maze-backprop-candle-wasm"
template = "about.html"
+++

<style>
    body {
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
        max-width: 800px;
        margin: 0 auto;
        padding: 20px;
        background-color: #f5f5f5;
        line-height: 1.6;
    }

    h1 {
        text-align: center;
        color: #333;
        margin-bottom: 30px;
    }

    #info {
        text-align: center;
        font-size: 18px;
        font-weight: bold;
        color: #2c3e50;
        margin-bottom: 20px;
        padding: 10px;
        background-color: #ecf0f1;
        border-radius: 5px;
        border-left: 4px solid #3498db;
    }

    #maze {
        display: block;
        margin: 0 auto 20px auto;
        border: 2px solid #34495e;
        border-radius: 5px;
        box-shadow: 0 4px 8px rgba(0,0,0,0.1);
        background-color: white;
        max-width: 100%;
        max-height: 60vh;
    }

    #controls {
        background-color: white;
        padding: 20px;
        border-radius: 10px;
        box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        margin-top: 20px;
    }

    .control-group {
        margin-bottom: 15px;
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .control-group label {
        font-weight: 600;
        color: #2c3e50;
        margin-right: 10px;
    }

    input[type="number"], input[type="range"] {
        padding: 5px 10px;
        border: 2px solid #bdc3c7;
        border-radius: 4px;
        font-size: 14px;
        transition: border-color 0.3s;
    }

    input[type="number"]:focus, input[type="range"]:focus {
        outline: none;
        border-color: #3498db;
    }

    input[type="number"] {
        width: 80px;
    }

    input[type="range"] {
        width: 150px;
    }

    #densityValue {
        font-weight: bold;
        color: #e74c3c;
    }

    button {
        width: 100%;
        padding: 12px 20px;
        font-size: 16px;
        font-weight: bold;
        color: white;
        background-color: #3498db;
        border: none;
        border-radius: 5px;
        cursor: pointer;
        transition: background-color 0.3s, transform 0.1s;
        margin-top: 10px;
    }

    button:hover:not(:disabled) {
        background-color: #2980b9;
        transform: translateY(-1px);
    }

    button:disabled {
        background-color: #95a5a6;
        cursor: not-allowed;
        transform: none;
    }

    @media (max-width: 600px) {
        body {
            padding: 10px;
        }

        .control-group {
            flex-direction: column;
            align-items: flex-start;
        }

        .control-group label {
            margin-bottom: 5px;
        }
    }
</style>

<div id="info">Loading WASM module...</div>
<canvas id="maze"></canvas>
<div id="controls">
    <div class="control-group">
        <label for="width">Width:</label>
        <input type="number" id="width">
    </div>
    <div class="control-group">
        <label for="height">Height:</label>
        <input type="number" id="height">
    </div>
    <div class="control-group">
        <label for="density">Wall Density: <span id="densityValue"></span></label>
        <input type="range" id="density">
    </div>
    <div class="control-group">
        <label for="learningRate">Learning Rate: <span id="learningRateValue"></span></label>
        <input type="range" id="learningRate">
    </div>
    <div class="control-group">
        <label for="maxSteps">Max Steps: <span id="maxStepsValue"></span></label>
        <input type="range" id="maxSteps">
    </div>
    <button id="generate" onclick="generateAndOptimize()">Generate & Optimize</button>
</div>

<script type="module">
    // Configuration constants - single source of truth
    const CONFIG = {
        width: { default: 10, min: 2, max: 20 },
        height: { default: 10, min: 2, max: 20 },
        wallDensity: { default: 0.3, min: 0.1, max: 0.5, step: 0.05 },
        learningRate: { default: 0.005, min: 0.0001, max: 1, step: 0.0001 },
        maxSteps: { default: 1000, min: 100, max: 5000, step: 100 },
        convergenceThreshold: 0.98, // Stop when goal probability reaches this
        canvasSize: 2048,
        cellPadding: 10
    };

    let animationId = null;
    let worker = null;
    let currentMazeInfo = null;

    // Initialize HTML elements from CONFIG
    function initializeControls() {
        // Width
        const widthInput = document.getElementById('width');
        widthInput.min = CONFIG.width.min;
        widthInput.max = CONFIG.width.max;
        widthInput.value = CONFIG.width.default;

        // Height
        const heightInput = document.getElementById('height');
        heightInput.min = CONFIG.height.min;
        heightInput.max = CONFIG.height.max;
        heightInput.value = CONFIG.height.default;

        // Density
        const densityInput = document.getElementById('density');
        densityInput.min = CONFIG.wallDensity.min;
        densityInput.max = CONFIG.wallDensity.max;
        densityInput.step = CONFIG.wallDensity.step;
        densityInput.value = CONFIG.wallDensity.default;
        document.getElementById('densityValue').textContent = CONFIG.wallDensity.default;

        // Learning rate
        const lrInput = document.getElementById('learningRate');
        lrInput.min = CONFIG.learningRate.min;
        lrInput.max = CONFIG.learningRate.max;
        lrInput.step = CONFIG.learningRate.step;
        lrInput.value = CONFIG.learningRate.default;
        document.getElementById('learningRateValue').textContent = CONFIG.learningRate.default;

        // Max steps
        const maxStepsInput = document.getElementById('maxSteps');
        maxStepsInput.min = CONFIG.maxSteps.min;
        maxStepsInput.max = CONFIG.maxSteps.max;
        maxStepsInput.step = CONFIG.maxSteps.step;
        maxStepsInput.value = CONFIG.maxSteps.default;
        document.getElementById('maxStepsValue').textContent = CONFIG.maxSteps.default;
    }

    // Update density display
    document.getElementById('density').addEventListener('input', (e) => {
        document.getElementById('densityValue').textContent = e.target.value;
    });

    // Update learning rate display
    document.getElementById('learningRate').addEventListener('input', (e) => {
        document.getElementById('learningRateValue').textContent = e.target.value;
    });

    // Update max steps display
    document.getElementById('maxSteps').addEventListener('input', (e) => {
        document.getElementById('maxStepsValue').textContent = e.target.value;
    });

    function setControlsEnabled(enabled) {
        const controls = ['width', 'height', 'density', 'learningRate', 'maxSteps', 'generate'];
        controls.forEach(id => {
            const elem = document.getElementById(id);
            if (elem) elem.disabled = !enabled;
        });

        // Add/remove overlay styling
        const controlsDiv = document.getElementById('controls');
        const infoDiv = document.getElementById('info');
        if (enabled) {
            controlsDiv.style.opacity = '1';
            controlsDiv.style.pointerEvents = 'auto';
            infoDiv.style.backgroundColor = '#ecf0f1';
            infoDiv.style.borderLeftColor = '#3498db';
        } else {
            controlsDiv.style.opacity = '0.6';
            controlsDiv.style.pointerEvents = 'none';
            infoDiv.style.backgroundColor = '#fff3cd';
            infoDiv.style.borderLeftColor = '#f39c12';
        }
    }

    async function init() {
        const startTime = performance.now();
        console.log(`[${startTime.toFixed(2)}ms] Starting initialization`);

        try {
            document.getElementById('info').textContent = 'Initializing worker...';

            // Create worker
            const workerCreateStart = performance.now();
            worker = new Worker('./training-worker.js');
            console.log(`[${(performance.now() - startTime).toFixed(2)}ms] Worker created (took ${(performance.now() - workerCreateStart).toFixed(2)}ms)`);

            // Set up worker message handler
            worker.onmessage = (e) => {
                switch (e.data.type) {
                    case 'wasm-loaded':
                        console.log(`[${(performance.now() - startTime).toFixed(2)}ms] WASM loaded in worker`);
                        document.getElementById('info').textContent = 'WASM loaded, generating maze...';
                        generateAndOptimize();
                        break;
                    case 'error':
                        document.getElementById('info').textContent = 'Error: ' + e.data.message;
                        console.error(e.data.message);
                        break;
                    case 'timing':
                        console.log(`[Worker] ${e.data.message}`);
                        break;
                }
            };

            // Initialize WASM in worker
            console.log(`[${(performance.now() - startTime).toFixed(2)}ms] Sending init-wasm message to worker`);
            worker.postMessage({ type: 'init-wasm' });
        } catch (error) {
            document.getElementById('info').textContent = 'Error creating worker: ' + error.message;
            console.error(error);
        }
    }

    window.generateAndOptimize = async function() {
        const genStartTime = performance.now();
        console.log(`[${genStartTime.toFixed(2)}ms] Starting generateAndOptimize`);

        if (!worker) {
            document.getElementById('info').textContent = 'Worker not initialized';
            return;
        }

        // Stop any ongoing animation
        if (window.stopAnimation) {
            window.stopAnimation();
        }

        // Get parameters
        const width = parseInt(document.getElementById('width').value);
        const height = parseInt(document.getElementById('height').value);
        const wallProb = parseFloat(document.getElementById('density').value);
        const learningRate = parseFloat(document.getElementById('learningRate').value);

        // Disable controls during training
        setControlsEnabled(false);
        document.getElementById('info').textContent = 'Initializing session...';

        // Promise to wait for session initialization
        const sessionPromiseStart = performance.now();
        const sessionReady = new Promise((resolve, reject) => {
            const handler = (e) => {
                if (e.data.type === 'session-initialized') {
                    console.log(`[${(performance.now() - genStartTime).toFixed(2)}ms] Session initialized in worker`);
                    worker.removeEventListener('message', handler);
                    currentMazeInfo = e.data.mazeInfo;
                    resolve();
                } else if (e.data.type === 'error') {
                    worker.removeEventListener('message', handler);
                    reject(new Error(e.data.message));
                }
            };
            worker.addEventListener('message', handler);
        });

        // Initialize session in worker
        console.log(`[${(performance.now() - genStartTime).toFixed(2)}ms] Sending init-session to worker`);
        worker.postMessage({
            type: 'init-session',
            width,
            height,
            wallProb,
            learningRate
        });

        try {
            await sessionReady;

            // Extract maze info
            const extractStart = performance.now();
            const mazeWidth = currentMazeInfo[0];
            const mazeHeight = currentMazeInfo[1];
            const startIdx = currentMazeInfo[2];
            const goalIdx = currentMazeInfo[3];

            // Extract maze structure
            const maze = [];
            for (let i = 0; i < mazeHeight; i++) {
                const row = [];
                for (let j = 0; j < mazeWidth; j++) {
                    row.push(currentMazeInfo[4 + i * mazeWidth + j] === 1);
                }
                maze.push(row);
            }
            console.log(`[${(performance.now() - genStartTime).toFixed(2)}ms] Maze data extracted (took ${(performance.now() - extractStart).toFixed(2)}ms)`);

            // Start training in worker
            console.log(`[${(performance.now() - genStartTime).toFixed(2)}ms] Starting optimization in worker`);
            const maxSteps = parseInt(document.getElementById('maxSteps').value);
            worker.postMessage({
                type: 'start-training',
                maxSteps: maxSteps,
                convergenceThreshold: CONFIG.convergenceThreshold
            });

            // Start animation that polls worker
            console.log(`[${(performance.now() - genStartTime).toFixed(2)}ms] Starting animation`);
            animateWithWorker(maze, mazeWidth, mazeHeight, startIdx, goalIdx);
        } catch (error) {
            setControlsEnabled(true);
            document.getElementById('info').textContent = 'Failed to create session: ' + error.message;
        }
    }

    function drawGreedyPath(ctx, maze, width, height, probs, logits, cellSize, padding) {
        const NUM_ACTIONS = 5;
        const ACTION_UP = 0;
        const ACTION_RIGHT = 1;
        const ACTION_DOWN = 2;
        const ACTION_LEFT = 3;
        const ACTION_NOOP = 4;

        // Parse logits into timesteps
        const timesteps = logits.length / NUM_ACTIONS;

        // Start from initial position (0, 0)
        let currentI = 0;
        let currentJ = 0;
        const path = [{i: currentI, j: currentJ}];

        // Simulate greedy policy execution
        for (let t = 0; t < timesteps; t++) {
            const idx = currentI * width + currentJ;

            // Get logits for current timestep
            const logitsStart = t * NUM_ACTIONS;
            const currentLogits = logits.slice(logitsStart, logitsStart + NUM_ACTIONS);

            // Find valid actions (not blocked by walls)
            const validActions = [];

            // Check each action
            if (currentI > 0 && !maze[currentI - 1][currentJ]) {
                validActions.push({action: ACTION_UP, logit: currentLogits[ACTION_UP]});
            }
            if (currentJ < width - 1 && !maze[currentI][currentJ + 1]) {
                validActions.push({action: ACTION_RIGHT, logit: currentLogits[ACTION_RIGHT]});
            }
            if (currentI < height - 1 && !maze[currentI + 1][currentJ]) {
                validActions.push({action: ACTION_DOWN, logit: currentLogits[ACTION_DOWN]});
            }
            if (currentJ > 0 && !maze[currentI][currentJ - 1]) {
                validActions.push({action: ACTION_LEFT, logit: currentLogits[ACTION_LEFT]});
            }
            validActions.push({action: ACTION_NOOP, logit: currentLogits[ACTION_NOOP]});

            // Choose action with highest logit among valid actions
            if (validActions.length === 0) break;

            const bestAction = validActions.reduce((best, current) =>
                current.logit > best.logit ? current : best
            );

            // Apply action
            let newI = currentI;
            let newJ = currentJ;

            switch (bestAction.action) {
                case ACTION_UP:
                    newI = currentI - 1;
                    break;
                case ACTION_RIGHT:
                    newJ = currentJ + 1;
                    break;
                case ACTION_DOWN:
                    newI = currentI + 1;
                    break;
                case ACTION_LEFT:
                    newJ = currentJ - 1;
                    break;
                case ACTION_NOOP:
                    // Stay in place
                    break;
            }

            // Only move if the new position is different
            if (newI !== currentI || newJ !== currentJ) {
                currentI = newI;
                currentJ = newJ;
                path.push({i: currentI, j: currentJ});
            }

            // Stop if we reached the goal
            if (currentI === height - 1 && currentJ === width - 1) {
                break;
            }
        }

        // Draw the path
        ctx.strokeStyle = 'blue';
        ctx.lineWidth = cellSize * 0.1;
        ctx.lineCap = 'round';
        ctx.lineJoin = 'round';

        ctx.beginPath();
        for (let i = 0; i < path.length; i++) {
            const x = padding + path[i].j * cellSize + cellSize / 2;
            const y = padding + path[i].i * cellSize + cellSize / 2;

            if (i === 0) {
                ctx.moveTo(x, y);
            } else {
                ctx.lineTo(x, y);
            }
        }
        ctx.stroke();
    }

    function animateWithWorker(maze, width, height, startIdx, goalIdx) {
        const animStart = performance.now();
        const canvas = document.getElementById('maze');
        const ctx = canvas.getContext('2d');
        const canvasSide = CONFIG.canvasSize;
        const cellSize = canvasSide / Math.max(width, height);
        const padding = CONFIG.cellPadding;

        canvas.width = width * cellSize + 2 * padding;
        canvas.height = height * cellSize + 2 * padding;

        let isAnimating = true;
        const maxSteps = parseInt(document.getElementById('maxSteps').value);
        let firstFrameDrawn = false;

        function drawFrame(timestamp) {
            // Check if we should continue animating
            if (!isAnimating || !worker) {
                setControlsEnabled(true);
                return;
            }

            // Request current state from worker
            worker.postMessage({ type: 'get-state' });

            // Handle the response
            const stateHandler = (e) => {
                if (e.data.type !== 'state') return;

                worker.removeEventListener('message', stateHandler);

                const { currentStep, probs, loss, logits, isComplete, reason } = e.data;

                // Draw current state if we have probabilities
                if (probs && probs.length > 0) {
                    if (!firstFrameDrawn) {
                        console.log(`[${(performance.now() - animStart).toFixed(2)}ms] First frame with data, drawing canvas`);
                        firstFrameDrawn = true;
                    }

                    // Clear canvas
                    ctx.fillStyle = 'white';
                    ctx.fillRect(0, 0, canvas.width, canvas.height);

                    // Draw maze cells
                    for (let i = 0; i < height; i++) {
                        for (let j = 0; j < width; j++) {
                            const x = j * cellSize + padding;
                            const y = i * cellSize + padding;

                            // Draw wall or free space
                            if (maze[i][j]) {
                                ctx.fillStyle = 'black';
                                ctx.fillRect(x, y, cellSize, cellSize);
                            } else {
                                // Draw probability heatmap
                                const idx = i * width + j;
                                const prob = probs[idx];
                                const intensity = Math.min(1, prob * 5);

                                // White to red gradient
                                const r = 255;
                                const g = Math.floor(255 * (1 - intensity));
                                const b = Math.floor(255 * (1 - intensity));
                                ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
                                ctx.fillRect(x, y, cellSize, cellSize);
                            }

                            // Draw grid lines
                            ctx.strokeStyle = '#ddd';
                            ctx.strokeRect(x, y, cellSize, cellSize);
                        }
                    }

                    // Draw greedy policy path if we have logits
                    if (logits && logits.length > 0) {
                        drawGreedyPath(ctx, maze, width, height, probs, logits, cellSize, padding);
                    }

                    // Draw start and goal
                    ctx.font = `bold ${Math.floor(cellSize * 0.6)}px sans-serif`;
                    ctx.textAlign = 'center';
                    ctx.textBaseline = 'middle';

                    // Start
                    const startJ = startIdx % width;
                    const startI = Math.floor(startIdx / width);
                    ctx.fillStyle = 'darkgreen';
                    ctx.fillText('S', padding + startJ * cellSize + cellSize/2,
                                padding + startI * cellSize + cellSize/2);

                    // Goal
                    const goalJ = goalIdx % width;
                    const goalI = Math.floor(goalIdx / width);
                    ctx.fillStyle = 'purple';
                    ctx.fillText('G', padding + goalJ * cellSize + cellSize/2,
                                padding + goalI * cellSize + cellSize/2);

                    // Update info
                    document.getElementById('info').textContent =
                        `Step: ${currentStep}/${maxSteps} | Loss: ${loss.toFixed(4)} | Goal Prob: ${(-loss).toFixed(4)}`;
                }

                // Check if training is complete
                if (isComplete) {
                    isAnimating = false;
                    setControlsEnabled(true);
                    worker.postMessage({ type: 'stop-training' });

                    if (reason === 'max-steps') {
                        document.getElementById('info').textContent += ' | Max steps reached';
                    } else if (reason === 'converged') {
                        document.getElementById('info').textContent += ' | Converged!';
                    }
                } else if (isAnimating) {
                    // Continue animation
                    animationId = requestAnimationFrame(drawFrame);
                }
            };

            worker.addEventListener('message', stateHandler);
        }

        // Store a reference to stop the animation when needed
        window.stopAnimation = () => {
            isAnimating = false;
            if (animationId) {
                cancelAnimationFrame(animationId);
                animationId = null;
            }
            if (worker) {
                worker.postMessage({ type: 'stop-training' });
            }
            setControlsEnabled(true);
        };

        // Start animation
        document.getElementById('info').textContent = 'Optimizing...';
        animationId = requestAnimationFrame(drawFrame);
    }

    // Handle window resize
    let resizeTimeout;
    window.addEventListener('resize', () => {
        clearTimeout(resizeTimeout);
        resizeTimeout = setTimeout(() => {
            if (worker && !document.getElementById('generate').disabled) {
                generateAndOptimize();
            }
        }, 250);
    });

    // Initialize immediately when script loads
    console.log(`[0.00ms] Page script loaded, initializing controls and calling init()`);
    initializeControls();
    init().catch(console.error);
</script>


This demo uses gradient descent to solve a discrete maze.

**Try playing with the hyperparameters** to see how they affect the optimization process!

**No neural network involved**: logits are directly optimized, from a random initialization, for each maze.

**This runs entirely on your local device**, thanks to [candle](https://github.com/huggingface/candle) and [Rust's support for WebAssembly](https://rustwasm.github.io/book/).
You can disconnect from the Internet and you will still be able to use this demo!

**Appearances can be deceiving**: On harder and larger grids, you might find that much time is spent being "stuck", with a dramatic phase transition. Beware! And perhaps try increasing the step count.

[This demo's code is available on GitHub.](https://github.com/yberreby/yberreby.github.io/tree/master/content/pages/discrete-maze-backprop-candle-wasm)
