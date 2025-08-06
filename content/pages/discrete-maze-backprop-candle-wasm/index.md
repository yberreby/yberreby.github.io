+++
title = "Backpropagating through a maze with candle and WASM"
path = "discrete-maze-backprop-candle-wasm"
template = "page.html"
date = "2025-08-06"
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
        font-size: 26px;
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

    #game-container {
        display: flex;
        gap: 10px;
        margin: 0 auto 20px auto;
        align-items: flex-start;
        justify-content: center;
        max-width: 800px;
    }

    #maze-container {
        flex: 0 1 auto;
    }

    #maze {
        display: block;
        border: 2px solid #34495e;
        border-radius: 5px;
        box-shadow: 0 4px 8px rgba(0,0,0,0.1);
        background-color: white;
        max-height: 60vh;
        width: auto;
        height: auto;
    }

    #logits-container {
        flex: 0 0 auto;
    }

    #logits {
        display: none;
        border: 2px solid #34495e;
        border-radius: 5px;
        box-shadow: 0 4px 8px rgba(0,0,0,0.1);
        background-color: white;
        image-rendering: pixelated;
        image-rendering: -moz-crisp-edges;
        image-rendering: crisp-edges;
        width: auto;
        height: auto;
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

        #game-container {
            flex-direction: column;
            align-items: center;
        }

        #maze-container, #logits-container {
            flex: 1 1 auto;
            width: 100%;
            justify-content: center;
        }
    }
</style>

<div id="info">Loading WASM module...</div>
<div id="game-container">
    <div id="maze-container">
        <canvas id="maze"></canvas>
    </div>
    <div id="logits-container">
        <canvas id="logits"></canvas>
    </div>
</div>
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

    function drawLogitsHeatmap(logits, mazeCanvasHeight) {
        const NUM_ACTIONS = 5;
        const canvas = document.getElementById('logits');
        const ctx = canvas.getContext('2d');

        if (!logits || logits.length === 0) {
            canvas.style.display = 'none';
            return;
        }

        canvas.style.display = 'block';

        // Calculate dimensions
        const timesteps = logits.length / NUM_ACTIONS;
        const padding = 10;

        // Canvas resolution - higher for crisp rendering
        const cellWidth = 60;
        const cellHeight = Math.max(6, Math.min(30, 600 / timesteps));

        canvas.width = NUM_ACTIONS * cellWidth + 2 * padding;
        canvas.height = timesteps * cellHeight + 2 * padding;

        // CSS display size - match maze height but narrower width
        canvas.style.height = mazeCanvasHeight + 'px';
        canvas.style.width = (canvas.width * mazeCanvasHeight / canvas.height / 3) + 'px';

        // Clear canvas
        ctx.fillStyle = 'white';
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        // Find min and max for normalization
        let minLogit = Infinity;
        let maxLogit = -Infinity;
        for (const logit of logits) {
            minLogit = Math.min(minLogit, logit);
            maxLogit = Math.max(maxLogit, logit);
        }
        const range = maxLogit - minLogit || 1;

        // Batlow colormap from cmcrameri
        function batlow(t) {
            // Clamp t to [0, 1]
            t = Math.max(0, Math.min(1, t));

            // Batlow colormap data (256 values)
            const cmap = [
                [0.004637, 0.098343, 0.349833], [0.008580, 0.104559, 0.350923], [0.012565, 0.110825, 0.351981], [0.016171, 0.116932, 0.353057],
                [0.019623, 0.122982, 0.354106], [0.022916, 0.129014, 0.355168], [0.026056, 0.135014, 0.356195], [0.029046, 0.140931, 0.357225],
                [0.031891, 0.146753, 0.358229], [0.034696, 0.152562, 0.359228], [0.037367, 0.158357, 0.360219], [0.039804, 0.164072, 0.361200],
                [0.042104, 0.169711, 0.362175], [0.044107, 0.175274, 0.363120], [0.045968, 0.180761, 0.364057], [0.047742, 0.186205, 0.364976],
                [0.049465, 0.191514, 0.365883], [0.050890, 0.196766, 0.366763], [0.052254, 0.201845, 0.367610], [0.053547, 0.206876, 0.368458],
                [0.054774, 0.211752, 0.369266], [0.055952, 0.216510, 0.370049], [0.057021, 0.221141, 0.370813], [0.057975, 0.225648, 0.371557],
                [0.059056, 0.230019, 0.372281], [0.060029, 0.234335, 0.372984], [0.060869, 0.238500, 0.373673], [0.061774, 0.242593, 0.374342],
                [0.062771, 0.246598, 0.374979], [0.063628, 0.250519, 0.375608], [0.064516, 0.254395, 0.376235], [0.065420, 0.258168, 0.376837],
                [0.066347, 0.261923, 0.377420], [0.067303, 0.265626, 0.377994], [0.068289, 0.269301, 0.378559], [0.069324, 0.272923, 0.379112],
                [0.070259, 0.276546, 0.379654], [0.071367, 0.280126, 0.380186], [0.072397, 0.283712, 0.380708], [0.073609, 0.287275, 0.381213],
                [0.074722, 0.290850, 0.381694], [0.075923, 0.294401, 0.382160], [0.077185, 0.297955, 0.382618], [0.078521, 0.301501, 0.383059],
                [0.079937, 0.305058, 0.383472], [0.081445, 0.308597, 0.383855], [0.082923, 0.312102, 0.384210], [0.084562, 0.315645, 0.384537],
                [0.086163, 0.319146, 0.384832], [0.087968, 0.322650, 0.385091], [0.089792, 0.326111, 0.385310], [0.091746, 0.329599, 0.385487],
                [0.093708, 0.333047, 0.385618], [0.095823, 0.336461, 0.385699], [0.098076, 0.339885, 0.385728], [0.100356, 0.343272, 0.385700],
                [0.102811, 0.346633, 0.385611], [0.105329, 0.349979, 0.385460], [0.107987, 0.353292, 0.385241], [0.110818, 0.356590, 0.384953],
                [0.113716, 0.359846, 0.384591], [0.116737, 0.363076, 0.384154], [0.119874, 0.366291, 0.383641], [0.123139, 0.369454, 0.383038],
                [0.126576, 0.372589, 0.382340], [0.130156, 0.375698, 0.381574], [0.133788, 0.378770, 0.380713], [0.137589, 0.381805, 0.379750],
                [0.141488, 0.384800, 0.378701], [0.145532, 0.387762, 0.377561], [0.149666, 0.390693, 0.376333], [0.153889, 0.393575, 0.374991],
                [0.158287, 0.396424, 0.373569], [0.162762, 0.399230, 0.372042], [0.167344, 0.402003, 0.370431], [0.172003, 0.404743, 0.368738],
                [0.176781, 0.407449, 0.366938], [0.181640, 0.410124, 0.365049], [0.186615, 0.412757, 0.363076], [0.191653, 0.415346, 0.361015],
                [0.196804, 0.417914, 0.358876], [0.201972, 0.420449, 0.356673], [0.207271, 0.422956, 0.354373], [0.212607, 0.425450, 0.351993],
                [0.218027, 0.427911, 0.349563], [0.223504, 0.430345, 0.347062], [0.229043, 0.432764, 0.344476], [0.234654, 0.435164, 0.341852],
                [0.240272, 0.437545, 0.339169], [0.245982, 0.439903, 0.336410], [0.251759, 0.442260, 0.333629], [0.257547, 0.444599, 0.330774],
                [0.263409, 0.446918, 0.327894], [0.269300, 0.449242, 0.324965], [0.275221, 0.451550, 0.322012], [0.281172, 0.453849, 0.319020],
                [0.287175, 0.456145, 0.316006], [0.293216, 0.458438, 0.312941], [0.299300, 0.460720, 0.309880], [0.305400, 0.463000, 0.306777],
                [0.311512, 0.465284, 0.303650], [0.317673, 0.467571, 0.300520], [0.323850, 0.469853, 0.297387], [0.330066, 0.472127, 0.294217],
                [0.336283, 0.474399, 0.291064], [0.342551, 0.476670, 0.287877], [0.348821, 0.478945, 0.284696], [0.355133, 0.481216, 0.281518],
                [0.361443, 0.483493, 0.278338], [0.367784, 0.485773, 0.275148], [0.374169, 0.488065, 0.271939], [0.380564, 0.490346, 0.268779],
                [0.386976, 0.492625, 0.265584], [0.393432, 0.494929, 0.262401], [0.399911, 0.497226, 0.259238], [0.406433, 0.499522, 0.256044],
                [0.412984, 0.501846, 0.252898], [0.419564, 0.504170, 0.249730], [0.426174, 0.506490, 0.246602], [0.432844, 0.508841, 0.243475],
                [0.439544, 0.511184, 0.240371], [0.446292, 0.513556, 0.237334], [0.453095, 0.515924, 0.234271], [0.459940, 0.518331, 0.231262],
                [0.466852, 0.520735, 0.228294], [0.473806, 0.523168, 0.225350], [0.480827, 0.525610, 0.222487], [0.487926, 0.528073, 0.219689],
                [0.495074, 0.530561, 0.216965], [0.502293, 0.533067, 0.214322], [0.509585, 0.535591, 0.211799], [0.516956, 0.538156, 0.209364],
                [0.524395, 0.540725, 0.207098], [0.531919, 0.543327, 0.204958], [0.539513, 0.545957, 0.202999], [0.547198, 0.548608, 0.201224],
                [0.554960, 0.551277, 0.199680], [0.562810, 0.553974, 0.198389], [0.570723, 0.556699, 0.197370], [0.578726, 0.559444, 0.196644],
                [0.586800, 0.562218, 0.196211], [0.594947, 0.564990, 0.196125], [0.603162, 0.567801, 0.196417], [0.611435, 0.570616, 0.197082],
                [0.619762, 0.573454, 0.198141], [0.628129, 0.576301, 0.199637], [0.636533, 0.579142, 0.201587], [0.644949, 0.581983, 0.204022],
                [0.653386, 0.584833, 0.206876], [0.661828, 0.587663, 0.210168], [0.670246, 0.590490, 0.213943], [0.678626, 0.593304, 0.218155],
                [0.686968, 0.596087, 0.222809], [0.695248, 0.598847, 0.227875], [0.703452, 0.601573, 0.233312], [0.711566, 0.604272, 0.239170],
                [0.719571, 0.606922, 0.245346], [0.727453, 0.609526, 0.251884], [0.735211, 0.612081, 0.258684], [0.742823, 0.614585, 0.265760],
                [0.750275, 0.617046, 0.273064], [0.757569, 0.619447, 0.280559], [0.764695, 0.621775, 0.288258], [0.771646, 0.624066, 0.296099],
                [0.778414, 0.626281, 0.304057], [0.785008, 0.628449, 0.312105], [0.791414, 0.630553, 0.320265], [0.797642, 0.632594, 0.328440],
                [0.803694, 0.634586, 0.336654], [0.809572, 0.636521, 0.344891], [0.815275, 0.638397, 0.353127], [0.820812, 0.640231, 0.361344],
                [0.826193, 0.642012, 0.369540], [0.831423, 0.643746, 0.377704], [0.836498, 0.645442, 0.385816], [0.841438, 0.647106, 0.393892],
                [0.846241, 0.648727, 0.401904], [0.850921, 0.650319, 0.409880], [0.855491, 0.651886, 0.417778], [0.859944, 0.653422, 0.425620],
                [0.864294, 0.654944, 0.433400], [0.868550, 0.656464, 0.441129], [0.872709, 0.657965, 0.448814], [0.876785, 0.659459, 0.456431],
                [0.880792, 0.660959, 0.464021], [0.884727, 0.662478, 0.471563], [0.888597, 0.664004, 0.479070], [0.892407, 0.665555, 0.486558],
                [0.896161, 0.667138, 0.494037], [0.899867, 0.668767, 0.501512], [0.903528, 0.670447, 0.508991], [0.907149, 0.672180, 0.516481],
                [0.910733, 0.673988, 0.524014], [0.914285, 0.675882, 0.531586], [0.917806, 0.677876, 0.539214], [0.921295, 0.679968, 0.546917],
                [0.924761, 0.682193, 0.554701], [0.928203, 0.684556, 0.562590], [0.931623, 0.687077, 0.570573], [0.935018, 0.689766, 0.578695],
                [0.938388, 0.692640, 0.586948], [0.941729, 0.695707, 0.595351], [0.945043, 0.698986, 0.603915], [0.948316, 0.702501, 0.612626],
                [0.951554, 0.706258, 0.621515], [0.954746, 0.710260, 0.630581], [0.957886, 0.714519, 0.639804], [0.960963, 0.719053, 0.649193],
                [0.963967, 0.723851, 0.658728], [0.966899, 0.728920, 0.668419], [0.969737, 0.734257, 0.678227], [0.972482, 0.739853, 0.688143],
                [0.975108, 0.745684, 0.698135], [0.977623, 0.751759, 0.708191], [0.980008, 0.758039, 0.718265], [0.982258, 0.764508, 0.728333],
                [0.984363, 0.771146, 0.738362], [0.986319, 0.777917, 0.748321], [0.988123, 0.784799, 0.758171], [0.989780, 0.791754, 0.767880],
                [0.991281, 0.798766, 0.777438], [0.992638, 0.805795, 0.786803], [0.993846, 0.812822, 0.795954], [0.994914, 0.819823, 0.804891],
                [0.995854, 0.826783, 0.813590], [0.996670, 0.833675, 0.822058], [0.997373, 0.840486, 0.830275], [0.997972, 0.847203, 0.838249],
                [0.998474, 0.853825, 0.845983], [0.998889, 0.860347, 0.853484], [0.999224, 0.866751, 0.860762], [0.999489, 0.873048, 0.867823],
                [0.999692, 0.879227, 0.874668], [0.999842, 0.885298, 0.881323], [0.999948, 0.891255, 0.887786], [1.000000, 0.897109, 0.894081],
                [1.000000, 0.902855, 0.900211], [1.000000, 0.908502, 0.906194], [1.000000, 0.914057, 0.912034], [1.000000, 0.919515, 0.917749],
                [1.000000, 0.924887, 0.923339], [1.000000, 0.930178, 0.928808], [1.000000, 0.935378, 0.934173], [1.000000, 0.940497, 0.939436],
                [1.000000, 0.945530, 0.944595], [1.000000, 0.950480, 0.949652], [1.000000, 0.955350, 0.954618], [1.000000, 0.960136, 0.959490],
                [1.000000, 0.964839, 0.964271], [1.000000, 0.969468, 0.968970], [1.000000, 0.974019, 0.973585], [1.000000, 0.978494, 0.978119],
                [1.000000, 0.982906, 0.982588], [1.000000, 0.987256, 0.986989], [1.000000, 0.991546, 0.991332], [1.000000, 0.995792, 0.995630]
            ];

            // Linear interpolation in the colormap
            const idx = t * (cmap.length - 1);
            const idx0 = Math.floor(idx);
            const idx1 = Math.min(idx0 + 1, cmap.length - 1);
            const frac = idx - idx0;

            const c0 = cmap[idx0];
            const c1 = cmap[idx1];

            return {
                r: Math.floor(255 * ((1 - frac) * c0[0] + frac * c1[0])),
                g: Math.floor(255 * ((1 - frac) * c0[1] + frac * c1[1])),
                b: Math.floor(255 * ((1 - frac) * c0[2] + frac * c1[2]))
            };
        }

        // Draw heatmap
        for (let t = 0; t < timesteps; t++) {
            for (let a = 0; a < NUM_ACTIONS; a++) {
                const logit = logits[t * NUM_ACTIONS + a];
                const normalized = (logit - minLogit) / range;

                const color = batlow(normalized);
                ctx.fillStyle = `rgb(${color.r}, ${color.g}, ${color.b})`;
                ctx.fillRect(
                    padding + a * cellWidth,
                    padding + t * cellHeight,
                    cellWidth - 1,
                    cellHeight - 1
                );
            }
        }

        // Draw grid lines
        ctx.strokeStyle = '#ddd';
        ctx.lineWidth = 1;
        for (let t = 0; t <= timesteps; t++) {
            ctx.beginPath();
            ctx.moveTo(padding, padding + t * cellHeight);
            ctx.lineTo(padding + NUM_ACTIONS * cellWidth, padding + t * cellHeight);
            ctx.stroke();
        }
        for (let a = 0; a <= NUM_ACTIONS; a++) {
            ctx.beginPath();
            ctx.moveTo(padding + a * cellWidth, padding);
            ctx.lineTo(padding + a * cellWidth, padding + timesteps * cellHeight);
            ctx.stroke();
        }

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
                        drawLogitsHeatmap(logits, canvas.offsetHeight);
                    } else {
                        // No logits yet - hide the canvas
                        const logitsCanvas = document.getElementById('logits');
                        if (logitsCanvas) logitsCanvas.style.display = 'none';
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
You can disconnect from the Internet after loading this demo and you will still be able to use it!

**Appearances can be deceiving**: On harder and larger grids, you might find that much time is spent being "stuck", with a dramatic phase transition. Beware! And perhaps try increasing the step count.

## Additional Details

This demo solves maze navigation by relaxing the discrete problem into a stochastic formulation that happens to be end-to-end differentiable.

Since every operation is differentiable, we use backpropagation with standard automatic differentiation (i.e. [candle](https://github.com/huggingface/candle)'s autograd, which runs client-side) to directly optimize action logits, without relying on e.g. the REINFORCE algorithm, Q learning, Monte-Carlo rollouts, or any sort of neural network.

### State Representation

There is no "current position" in a discrete sense.

Instead, at any given point, there is a _probability distribution over possible positions_.

The agent's state is a 2D probability grid \\( s_t \in \mathbb{R}^{H \times W} \\) where:
- \\( s_t[r,c] \\) is the probability of being at row \\( r \\), column \\( c \\)
- \\( \sum_{r=0}^{H-1} \sum_{c=0}^{W-1} s_t[r,c] = 1 \\)

This allows us to apply gradient-based optimization to this inherently-discrete problem.
By smoothly varying the action distribution, we can smoothly increase our probability of success.

### Action Space

At each time step, there are five possible actions:

\\[ \mathcal{A} = \\{\text{up}, \text{right}, \text{down}, \text{left}, \text{noop}\\}. \\]

The \\( \text{noop} \\) action, which represents staying in place, is important in this simple demo.

Without it, one would be forced to take a step at each time step, even if the goal was already reached.

### Transition Dynamics
For each action \\( a \\), we pre-compute transitions between grid positions:
\\[ T^{(a)}_{(r,c) \to (r',c')} = \begin{cases} 1 & \text{if action } a \text{ moves from } (r,c) \text{ to } (r',c') \\\\ 0 & \text{otherwise} \end{cases} \\]

The actions map to movements:
- **up**: \\( (r,c) \to (r-1,c) \\)
- **right**: \\( (r,c) \to (r,c+1) \\)
- **down**: \\( (r,c) \to (r+1,c) \\)
- **left**: \\( (r,c) \to (r,c-1) \\)
- **noop**: \\( (r,c) \to (r,c) \\)

Impossible actions (walls or boundaries) loop back: \\( T^{(a)}_{(r,c) \to (r,c)} = 1 \\).

Yes, this matrix is _enormous_, and it's mostly sparse.

Why use it, then?

Because it allows us to apply actions using matrix multiplication, and thus to keep autograd happy with minimal hassle.

### Direct Logit Parameterization

We use _position-independent action probabilities_ that produce _position-dependent behavior_ through masking.

Our parameters are time-dependent logits:
\\[ \theta_t \in \mathbb{R}^{|\mathcal{A}|} \\]

This is a tensor of shape \\( T \times |\mathcal{A}| = T \times 5 \\), where \\( T \\) is the maximum number of time steps.
It's represented by the grid that you see evolve on the right-hand side of the maze above.

The parameters \\( \theta_t \\) don't know anything about positions.
They're the same for every cell in the maze at time \\( t \\).


Does this seem like it shouldn't work? I would agree! I definitely would _not_ parameterize things this way in a serious setting.

For this demo, it is enough to show interesting behavior.

### State Evolution
At each timestep, probability mass flows according to:
\\[ s_{t+1}[r',c'] = \sum_{r=0}^{H-1} \sum_{c=0}^{W-1} \sum_{a \in \mathcal{A}} s_t[r,c] \cdot \pi_t(a|r,c) \cdot T^{(a)}_{(r,c) \to (r',c')} \\]

In other words: the probability of being at position \\( (r',c') \\) next is the sum over all ways to get there. For each possible starting position \\( (r,c) \\), we take the probability \\( s_t[r,c] \\) of being there, multiply by the probability \\( \pi_t(a|r,c) \\) of taking action \\( a \\), and multiply by \\( T^{(a)}_{(r,c) \to (r',c')} \\) which is 1 if action \\( a \\) moves you from \\( (r,c) \\) to \\( (r',c') \\) and 0 otherwise.

Here, position dependence emerges (we do need it somewhere): we apply a masked softmax using the same \\( \theta_t \\) at every position, but only over valid actions:
\\[ \pi_t(a|r,c) = \begin{cases} \frac{\exp(\theta_t[a])}{\sum_{a' \in V_{r,c}} \exp(\theta_t[a'])} & \text{if } a \in V_{r,c} \\\\ 0 & \text{otherwise} \end{cases} \\]

where \\( V_{r,c} \\) is the set of valid (non-wall-blocked) actions from position \\( (r,c) \\).

This creates an effective position-dependent policy \\( \pi_t(a|r,c) \\) from position-independent parameters \\( \theta_t \\).

### Learning Objective

We simply maximize the probability of reaching the goal (the bottom-right cell) after \\( T \\) steps:
\\[ \mathcal{L} = -s_T[H-1, W-1] \\]

The time horizon adapts to maze difficulty: \\( T = 2 \times \text{shortest\\_path\\_length} \\).

### Algorithm
1. Initialize \\( s_0[0,0] = 1 \\) and \\( s_0[r,c] = 0 \\) elsewhere (start at top-left)
2. Initialize parameters \\( \theta_1, \ldots, \theta_T \sim \mathcal{N}(0, 0.1) \\)
3. For each gradient step:
   - Compute loss \\( \mathcal{L} = -s_T[H-1, W-1] \\)
   - Update parameters via the [Adam](https://arxiv.org/abs/1412.6980) optimizer
   - Stop if the probability of reaching the goal exceeds a predefined threshold.

## Just give me the code

[This demo's code is available on GitHub](https://github.com/yberreby/yberreby.github.io/blob/master/content/pages/discrete-maze-backprop-candle-wasm/code/src/lib.rs), in case you, too, wish to do client-side backpropagation in Rust.
