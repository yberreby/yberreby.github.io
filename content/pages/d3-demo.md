+++
title = "D3 demo"
path = "d3-demo"
template = "about.html"
+++


Hello!

{% katex(block=true) %}\KaTeX{% end %}

<div id="chart"></div>
<div>
    <label for="frequency">Frequency: </label>
    <input type="range" id="frequency" min="1" max="10" step="0.1" value="1">
</div>

<script src="https://d3js.org/d3.v7.min.js"></script>
<script>
    // Set up the SVG canvas
    const margin = {top: 10, right: 30, bottom: 30, left: 30},
        width = 500 - margin.left - margin.right,
        height = 200 - margin.top - margin.bottom;

    const svg = d3.select("#chart")
        .append("svg")
        .attr("width", width + margin.left + margin.right)
        .attr("height", height + margin.top + margin.bottom)
        .append("g")
        .attr("transform", `translate(${margin.left},${margin.top})`);

    // Create the x-axis (time) and y-axis (amplitude)
    const x = d3.scaleLinear()
        .domain([0, 2 * Math.PI])  // 0 to 2π for sine wave period
        .range([0, width]);

    const y = d3.scaleLinear()
        .domain([-1, 1])  // Amplitude range for sine wave
        .range([height, 0]);

    svg.append("g")
        .attr("transform", `translate(0, ${height})`)
        .call(d3.axisBottom(x).ticks(10));

    svg.append("g")
        .call(d3.axisLeft(y).ticks(5));

    // Line generator for sine wave
    const line = d3.line()
        .x(d => x(d[0]))
        .y(d => y(d[1]));

    // Generate sine wave data
    function generateSineWave(frequency) {
        const points = [];
        for (let i = 0; i <= 1000; i++) {
            const t = (i / 1000) * 2 * Math.PI;
            points.push([t, Math.sin(frequency * t)]);
        }
        return points;
    }

    // Initial plot with frequency 1
    let currentFrequency = 1;
    const sineData = generateSineWave(currentFrequency);

    // Add the sine wave path
    const path = svg.append("path")
        .datum(sineData)
        .attr("fill", "none")
        .attr("stroke", "steelblue")
        .attr("stroke-width", 2)
        .attr("d", line);

    // Function to update the sine wave
    function updateChart(frequency) {
        const newData = generateSineWave(frequency);
        path.datum(newData)
            .attr("d", line);
    }

    // Add slider interaction
    d3.select("#frequency").on("input", function() {
        currentFrequency = +this.value;
        updateChart(currentFrequency);
    });
</script>
