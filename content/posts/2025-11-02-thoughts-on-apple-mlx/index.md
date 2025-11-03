+++
title = "Some Thoughts on Apple's MLX"
+++

### Write Once, Run Nowhere?

Apple currently describes its [MLX](https://github.com/ml-explore/mlx) framework as "an array framework for machine learning **on Apple silicon**".
Given its quest for vertical integration and interest in AI,
it is unsurprising that Apple would rather come up with its own ML framework than depend on Meta's or Google's tooling.
It is natural to view MLX as yet another of Apple's proprietary tools, built to fit Apple's needs, not those of the ML community at large.
While MLX has automatic differentiation (autograd), and thus supports more than just inference, an Apple-Silicon-only framework could scarcely be more than a curiosity for ML practitioners who must target NVIDIA GPUs via CUDA, Google TPUs via [XLA](https://openxla.org/xla), or maybe the more exotic [AMD Instinct MI250X instances on Oak Ridge's exaFLOPS-scale Frontier supercomputer](https://docs.olcf.ornl.gov/systems/frontier_user_guide.html).

Why go through the delicate task of rewriting and re-optimizing your training and inference code for a variety of platforms if you can avoid it?
To make matters worse, as of writing, MLX [requires you to write your own `jacobian` function](https://github.com/ml-explore/mlx/discussions/671) and [has to defer to the CPU for matrix inversion](https://github.com/ml-explore/mlx/issues/1238).
That last one _hurts_.[^1]

That being said, the view of MLX as an Apple-only tool might end up becoming too narrow.
MLX is still in its infancy; primitives will be added as time passes.
It is also open source, thankfully.
[In June 2025, an early-stage CUDA backend for MLX was merged](https://news.ycombinator.com/item?id=44565668), which has garnered much attention from the community, as one could soon hope to run MLX code on NVIDIA GPUs. At least, in theory.
Will MLX end up offering serious, first-class support for CUDA, XLA, and others? Time will tell.
While I'm somewhat optimistic, I wouldn't hold my breath.

### Intriguing Peculiarities

Setting aside MLX's uncertain future on non-Apple platforms and the harrowing dearth of [GPU/Metal](https://developer.apple.com/metal/) support for "advanced" operations such as matrix inversion, MLX is still interesting.
For non-production uses, it can be a fun learning exercise to see how far one can get by composing the primitives that _are_ there or writing their own version of missing ones.
MLX also comes with a few interesting particularities, including [unified-memory-centric design](https://ml-explore.github.io/mlx/build/html/usage/unified_memory.html), [lazy evaluation](https://ml-explore.github.io/mlx/build/html/usage/lazy_evaluation.html), support for [graph compilation](https://ml-explore.github.io/mlx/build/html/usage/compile.html), and a JAX-inspired API that _isn't_ purely functional, [unlike JAX's own API](https://docs.jax.dev/en/latest/notebooks/Common_Gotchas_in_JAX.html#pure-functions).

Unified Memory (UM), in the Apple/everything-soldered sense of the term, purposefully blurs the lines between CPU and GPU memory.
While Apple can be credited with making high-bandwidth unified memory architectures relevant for ML, the competition has taken notice, notably AMD with their AI Max chips, which are being [explicitly marketed for consumer inference of local LLMs](https://www.amd.com/en/blogs/2025/amd-ryzen-ai-max-upgraded-run-up-to-128-billion-parameter-llms-lm-studio.html). [Strix Halo](https://news.ycombinator.com/item?id=43360894) is worth watching, and the new [Framework Desktop](https://frame.work/ca/en/desktop) will come with unified memory.
MLX might end up being an interesting choice on such platforms.

Lazy evaluation (i.e. not computing values until they must be materialized) is an intriguing choice, particularly combined with an imperative programming model, as is the case here.
I have yet to form a proper opinion on this choice, so I will defer to [the relevant documentation](https://ml-explore.github.io/mlx/build/html/usage/lazy_evaluation.html) for now.

### Awkward Adolescence

In effect, MLX is in a slightly ambiguous position of both competing and _not_ competing with other deep-learning-oriented tensor programming libraries.
If one sees it as a contender, it is up against Python incumbents (Meta's [PyTorch](https://github.com/pytorch/pytorch), Google's [JAX](https://github.com/jax-ml/jax/)) and challengers (tiny corp's [tinygrad](https://tinygrad.org/)), and perhaps even Rust frameworks like HuggingFace's [candle](https://github.com/huggingface/candle), or [Tracel AI](https://tracel.ai/)'s [burn](https://github.com/tracel-ai/burn).

In many production use cases, right now, MLX is the wrong choice.
It is wildly immature compared to PyTorch and JAX, missing critical tooling, primitives, and features; it lacks the WebAssembly support that makes [candle](https://github.com/huggingface/candle) or [burn](https://github.com/tracel-ai/burn/) so attractive; and for now, it just doesn't work on non-Apple tensor hardware.

Where it might be most interesting, however, is for writing small and fast ML code meant to run locally on Apple Silicon. To some extent, this has been possible through `jax-metal` ([PyPI](https://pypi.org/project/jax-metal/); [Apple's docs](https://developer.apple.com/metal/jax/)) and PyTorch's MPS bindings, but both are missing features / fast kernels for a number of primitives. Moreover, `jax-metal` [hasn't been updated in a long time](https://github.com/jax-ml/jax/issues/26968), currently only supporting JAX up to [v0.5.0, released on January 17, 2025](https://github.com/jax-ml/jax/releases/tag/jax-v0.5.0). [JAX v0.8.0 came out on October 15, 2025](https://github.com/jax-ml/jax/releases/tag/jax-v0.8.0).

I sadly suspect that Apple will be deprioritizing `jax-metal` going forward, and, likewise, I am not confident in the continued support for PyTorch's MPS bindings.
At the moment, MLX seems to be the best-supported way to use MPS without writing low-level kernels by hand.


[^1]: One might say, "why would you invert a matrix anyway? You should be solving a linear system!" Well, [that still won't work](https://github.com/ml-explore/mlx/issues/847). Right now, [`mlx.core.linalg.solve`](https://ml-explore.github.io/mlx/build/html/python/_autosummary/mlx.core.linalg.solve.html) also runs on CPU, and so does [`mlx.core.linalg.svd`](https://ml-explore.github.io/mlx/build/html/python/_autosummary/mlx.core.linalg.svd.html).
