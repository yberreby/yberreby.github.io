+++
title = "StrangeDraw: Painting with Chaos"
draft = false
+++

_This is an early version of this post; it will be updated with more details in the future._

Did you know that you can combine [optimal transport / transport theory](https://en.wikipedia.org/wiki/Transportation_theory_(mathematics)) with [chaos theory](https://en.wikipedia.org/wiki/Chaos_theory), specifically [strange attractors](https://en.wikipedia.org/wiki/Attractor#Strange_attractor), in order to draw arbitrary shapes?

I always felt that reshaping the point densities of beautiful-looking strange attractors ([some examples here](https://paulbourke.net/fractals/lyapunov/)) should be possible in a rather straightforward manner.

I've toyed with a few ways to do that, including allowing particles to evolve in their original Euclidean space, in order to preserve their chaotic dynamics, while subjecting that space to a diffeomorphism in order to warp it such that the attractor's density matches that of the desired shape (typically binary: maximized in "filled" regions, and 0 elsewhere).

However, I have found it even more direct, yet effective, to use a [Sliced Wasserstein Distance](https://pythonot.github.io/auto_examples/sliced-wasserstein/plot_variance.html), in order to encourage the distribution formed by the particles to match the distribution defined by the target shape.

After getting this to work locally, I fed two of my experimental Python files to the freshly-released Claude 4.5 Opus, and asked it to make a Web-based, GPU-accelerated implementation of this idea.

**[Here is the result.](https://claude.ai/public/artifacts/fce3a033-27fa-4c9e-95dc-65765d0e8961)**

If you let the demo run, the particles will settle into the shape of the target text, which can be edited freely in real-time (try it!). I encourage you to play around with the system's various knobs; you can get very interesting-looking results by doing so.

I have decided to call this fun little side project "StrangeDraw", in honor of the strange attractors that inspired it and that give it texture. I find the idea of _drawing with chaos_ fascinating. If I can find the time and there is interest, I will do a proper GitHub release at some point in the future. In the meantime, I hope this sparked your interest as much as it did mine.
