+++
title = "Painting with Chaos"
draft = false
+++

Did you know that you can combine [optimal transport / transport theory](https://en.wikipedia.org/wiki/Transportation_theory_(mathematics)) with [chaos theory](https://en.wikipedia.org/wiki/Chaos_theory), specifically [strange attractors](https://en.wikipedia.org/wiki/Attractor#Strange_attractor), in order to draw arbitrary shapes?

I always felt that reshaping the point densities of beautiful-looking strange attractors ([some examples here from Paul Bourke's website](https://paulbourke.net/fractals/lyapunov/)) should be possible in a rather straightforward manner.[^a]

I mean, look at these examples from Bourke's page, and tell me they can't become a "T" and an "O" with a wee bit of imagination:

<div style="display:flex; gap:2%; justify-content:center; align-items:center; flex-wrap:nowrap; width:100%; margin:0 auto;">
  <img src="./bourke_inverted_t.jpg" style="flex:0 0 49%; height:20vh; object-fit:contain; display:block;" />
  <img src="./bourke_o.jpg"  style="flex:0 0 49%; height:20vh; object-fit:contain; display:block;" />
</div>

I've toyed with a few ways to bend chaos to my bidding.
This includes allowing particles to evolve in their original Euclidean space, in order to preserve the richness of the chosen attractor's dynamics, while subjecting that space to a diffeomorphism that warps it such that the attractor's density matches that of the desired shape (typically binary: maximized in "filled" regions, and 0 elsewhere).
That "works", but the results don't necessarily look as good as one would expect. It also tends to be a little slow, as one needs to learn the diffeomorphism (which can be done by training a small MLP on the fly).

There is a more direct and effective way to blend chaos and order: using gradient-based optimization on the particles' state vector.
In particular, minimizing a loss based on a [Sliced Wasserstein Distance](https://pythonot.github.io/auto_examples/sliced-wasserstein/plot_variance.html) can push the empirical distribution formed by the particles (each of which can be seen as a realization of a random variable over the 2D plane) to match the distribution defined by some target shape.

Here, the "sliced" aspect of SWD is meant to keep things fast enough to run in real-time; the \\( \mathcal{O}(n^2)\\) complexity of pairwise computation hits you really fast with 1,000-100,000 particles.

A key advantage of using gradient-based optimization on top of the original dynamics is that it's very easy to keep things looking _interesting_, and play around with the relative strength of the forces acting on the particles.
It also feels very natural when one remembers that the continuous analog of gradient descent, [gradient flow](https://rbcborealis.com/research-blogs/gradient-flow/#Gradient_flow), _is_ a dynamical system, just like the one that produces the original attractor.[^0]

At first, I just tried to reshape the particles into an "O", with a few other loss terms such as using a [signed distance field](https://en.wikipedia.org/wiki/Signed_distance_function) to penalize the distance to the shape's boundary (this turns out to be unnecessary).
My first attempts led to the discovery of very peculiar artifacts in extreme numerical regimes, which I turned into the following visualization (complete with audio—courtesy of [ElevenLabs](https://elevenlabs.io/)).

<iframe width="560" height="315" src="https://www.youtube.com/embed/_p-ggQkcSC8?si=mUKhjaHYroxxzspR" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

It starts "normal" (as normal as a strange attractor can be!), morphs into an "O" for a brief moment, then all hell breaks loose. Very trippy. LSD ain't got nothing on nonlinear dynamics; perhaps because it _is_ nonlinear dynamics all the way down? [The brain agrees](https://en.wikipedia.org/wiki/Hodgkin%E2%80%93Huxley_model).

After gleefully playing around with this glitch art and getting my proof-of-concept scripts to work locally, I fed two of my messy Python experiments, which used [Taichi](https://www.taichi-lang.org/) and PyTorch, to the freshly-released [Claude 4.5 Opus](https://www.anthropic.com/news/claude-opus-4-5), and asked it to make a Web-based, GPU-accelerated implementation of this idea.

**[Here is the result.](https://claude.ai/public/artifacts/fce3a033-27fa-4c9e-95dc-65765d0e8961)** It is pretty cool. I swear.
It is recommended to view it on a computer.

If you let the demo run, the particles will settle into the shape of the target text, which can be edited freely in real-time (try it!). The weighting of the transportation loss will then be pushed to excessive values, then brought back down, in an endless cycle. I encourage you to play around with the system's various knobs; you can get very interesting-looking results by doing so.

Some things to try: when the text is legible, type your own and watch the particles reshape themselves. Uncheck "auto-ramp" to play with the "SWD strength" slider yourself—that parameter is arguably the system's most important knob, as it controls the contribution of the _order_ imposed by the transportation loss.

Claude converted my Python-Taichi-Torch mess into a runnable Web-based demo from the very first attempt; beyond that, I re-prompted it a few times to add a few quality-of-life features.

It technically runs on mobile devices, but the sliders get squished.
While I have no doubt that it _could_ do it, coaxing Claude into turning this from "demo" to "production-ready webapp" will lead to sharply diminishing returns.
Still, not bad for an entirely vibe-coded port; Claude's Web interface won't even let me edit the code by hand, so every line had to come from the LLM.
I must say there is something refreshing about being able to feed a LLM the "hard" parts of a problem, and getting a readily-shareable React app out in less time than it would normally take to set up a build environment.
Is it nearly as polished as I would have made it by myself? Of course not.
But _would I have made it myself_, given the time investment? Maybe, maybe not; maybe much later.

I have decided to call this fun little side project "StrangeDraw" (or `strangedraw`—capitalization TBD), in honor of the strange attractors that inspired it and that give it texture. I find the idea of _drawing with chaos_ fascinating.

If I can find the time and there is interest, I might do a proper GitHub release at some point in the future, turning this from a one-off demo into a proper library. If you think this would be cool, please [shoot me an email](mailto:me@yberreby.com) or [open an issue here](https://github.com/yberreby/strangedraw/issues) with suggestions! I am debating how to approach it. Perhaps I will use it as an excuse to write some [Julia](https://julialang.org/) code[^1]. Or maybe I'll see how useable [IREE](https://iree.dev/) is for writing retargetable compute kernels using [MLIR Linalg](https://iree.dev/community/blog/2024-01-29-iree-mlir-linalg-tutorial/?hl=en-US#:~:text=The%20point%20of%20the%20above,intermediate%20representation%20in%20this%20compiler.)[^2].

In the meantime, feel free to read the code of the React demo I linked. I hope this sparked your interest as much as it did mine.

PS: Is this useful? No. Is it pretty? I think so.

[^a]: See [Shashank Tomar's _Show HN_ post from last month](https://news.ycombinator.com/item?id=45777810) for a discussion of the topic, and to play around with more beautiful-looking strange attractors right from your browser.

[^0]: Things do get a bit more hairy when you introduce [Adam](https://arxiv.org/abs/1412.6980), as I did, in order to get better convergence.

[^1]: Julia has the incredible [DynamicalSystems.jl](https://juliadynamics.github.io/DynamicalSystemsDocs.jl/dynamicalsystems/stable/) and [ChaosTools.jl](https://juliadynamics.github.io/DynamicalSystemsDocs.jl/chaostools/stable/), which have been drawing my attention ever since I suffered at the metaphorical hands of the mad yet brilliant [XPP/XPPAUT](https://sites.pitt.edu/~phase/bard/bardware/xpp/xpp.html).

[^2]: The promise of supporting CPU, Vulkan, CUDA, Metal and WebGPU all at once, for free, without hardcoding block sizes, seems very enticing. On top of that, did you know this intermediate representation is technically higher-level than OpenAI's [Triton](https://triton-lang.org/main/index.html)? That blew my mind.
