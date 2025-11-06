+++
title = "Policy Gradient Estimation in Deep RL"
draft = false
+++

_Disclaimer: This is an early version of this post._

When we do deep Reinforcement Learning (RL) with a [policy gradient](https://lilianweng.github.io/posts/2018-04-08-policy-gradient/) method[^0], we have a neural network _policy_ \\( \pi_\theta \\), and we're trying to optimize its _parameters_ \\(\theta\\) in order to maximize the _expected return_ \\( J(\pi_\theta) = \mathbb{E} _ { \tau \sim \pi_\theta } \left[ R(\tau) \right] \\), where \\( R(\tau) \\) is the _return_ of a trajectory \\(\tau\\).
In the case of _finite-horizon undiscounted return over \\( T+1 \\) timesteps_, we can write \\( R(\tau) = \sum_{t=0}^{T} r_t \\); for _infinite-horizon \\(\gamma\\)-discounted return_[^1b], we can write \\( R(\tau) = \sum_{t=0}^{\infty} \gamma^t r_t \\), with \\(\gamma \in (0, 1) \\).[^1]

There are many policy-gradient deep RL algorithms,
with various approaches
to the explore-exploit or bias-variance tradeoffs,
stability tricks,
data efficiency due to reusing stale rollouts,
hyperparameter choices,
etc.
However,
in order to use gradient-based optimization[^2],
they invariably estimate
_the gradient of the expected return with respect to the policy parameters_,
also known as **the policy gradient** \\(  \nabla_\theta J \\).

I find it extremely useful to categorize deep RL algorithms according to how they estimate \\( \nabla_\theta J \\).
Everything else is best understood in the context of its influence on this estimation.

If we do this, three main categories emerge:
- **[REINFORCE](https://en.wikipedia.org/wiki/Policy_gradient_method#REINFORCE)-based \\( \nabla_\theta J \\) estimation**: relies on the **[Policy Gradient Theorem (PGT)](https://web.stanford.edu/~ashlearn/RLForFinanceBook/PolicyGradient.pdf)**, using the [score function](https://en.wikipedia.org/wiki/Informant_(statistics))) / [log-derivative trick](https://andrewcharlesjones.github.io/journal/log-derivative.html) to compute an _unbiased_ (but high-variance) estimate of \\( \nabla_\theta J \\).
  - [VPG](https://spinningup.openai.com/en/latest/algorithms/vpg.html)/A2C/[A3C](https://arxiv.org/abs/1602.01783), [TRPO](https://spinningup.openai.com/en/latest/algorithms/trpo.html), and the widely-used/modern **[PPO](https://spinningup.openai.com/en/latest/algorithms/ppo.html)** and **[GRPO](https://arxiv.org/abs/2402.03300)**[^3] all rely on REINFORCE.
  - Key autograd-derived quantity: \\( \nabla_\theta \log \pi_\theta \\).
    - "How would changes to the policy parameters affect the log-likelihood of a given action?"
  - Requires a **stochastic** policy.
  - Remains unbiased even for categorical actions.
  - A value estimate is not structurally or theoretically required. However, in practice, baseline-free REINFORCE is incredibly slow to converge even on toy problems. This means that one usually trains a critic network to estimate the value function, in order to reduce variance.
  - While some sort of baseline is a no-brainer, don't be so quick to assume you need a critic network: in [DeepSeekMath's GRPO](https://arxiv.org/abs/2402.03300), LLMs undergo RL training at scale without a critic, using simple averaging across \\(k=64\\) rollouts from the same prompt to produce a good-enough baseline.
- **Critic-based \\( \nabla_\theta J \\) estimation**, AKA "backprop through a learned critic", yielding low-variance but biased (due to the critic's approximation being imperfect) gradient estimates.
  - This category includes [DDPG](https://spinningup.openai.com/en/latest/algorithms/ddpg.html), [TD3](https://spinningup.openai.com/en/latest/algorithms/td3.html), and [SAC](https://spinningup.openai.com/en/latest/algorithms/sac.html). SHAC, AHAC and SAPO (discussed in the next section) also partially rely on critic-based policy gradient estimation.
  - Key autograd-derived quantities: \\( \nabla_a Q \\) and \\( \nabla_\theta a \\)
    - \\( \nabla_a Q \\): "How would changes to the action affect my estimate of the state-action value function?"
    - \\( \nabla_\theta a \\): "How would changes to the policy parameters affect the (sampled or deterministic) action?"
    - By combining the above through the chain rule, we estimate \\( \nabla_\theta Q \\): "How would changes to the policy parameters affect my estimate of the state-action value function?"
  - Here, the learned critic network is not just a variance-reduction trick; it is structurally indispensable for training the actor!
  - This approach is compatible with **deterministic actions**, as seen in DPG/DDPG.
  - When using stochastic actions (as in SAC), one normally relies on **reparameterized sampling of actions** in order to let autograd compute \\( \nabla_\theta a \\).[^4]
- **Simply backpropagating through the environment**
  - Dead simple at its core: if the instantaneous reward and the dynamics are differentiable, we can just let autograd do the work, get an unbiased gradient estimate[^5], and maximize the empirical reward without doing anything special.
  - While this sounds great in theory (after all, why jump through hoops if autograd gives you unbiased gradients?), the resulting gradients can be very high-variance/norm, particularly at long horizons.
  - [APG](https://arxiv.org/abs/2209.13052), [SHAC](https://short-horizon-actor-critic.github.io/), [AHAC](https://adaptive-horizon-actor-critic.github.io/) and [SAPO](https://rewarped.github.io/) all rely on a _differentiable_ environment, which enables each of them to use backpropagation through time.
  - APG relies solely on BPTT; SHAC, AHAC and SAPO all combine BPTT with critic-based estimation at longer temporal horizons.
  - If you're interested, you should read [Do Differentiable Simulators Give Better Policy Gradients? (2022)](https://arxiv.org/abs/2202.00817).


[^0]: I will not discuss Deep Q-Learning (DQN) and other non-policy-gradient methods here, as they are out of scope for this post about deep policy gradient methods.

[^1]: For more details, see e.g. OpenAI's [Spinning Up in Deep RL - Part 1: Key Concepts in RL](https://spinningup.openai.com/en/latest/spinningup/rl_intro.html).

[^1b]: Consider that choosing \\( \gamma < 1 \\) still has a purpose in finite-horizon cases: it turns the [Bellman expectation backup operator](https://www.cs.cmu.edu/~rsalakhu/10703/Lectures/Lecture3_exactmethods.pdf) into a [contraction mapping](https://en.wikipedia.org/wiki/Contraction_mapping), enabling [bootstrapping](https://datascience.stackexchange.com/questions/26938/what-exactly-is-bootstrapping-in-reinforcement-learning) to work even with meaningless value estimates, as are produced early in training, before the critic has learned much.

[^2]: Via SGD, Adam(W), RMSProp, Muon, etc.
The undisputably-successful and still somewhat-recent [DreamerV3](https://danijar.com/project/dreamerv3/) (originally released in 2023) uses the [LaProp](https://arxiv.org/abs/2002.04839) optimizer (described as "RMSProp with momentum"), which I have never encountered elsewhere. [Muon](https://kellerjordan.github.io/posts/muon/) is now the default optimizer in [pufferlib](https://puffer.ai/) 3.0 ([relevant line of code](https://github.com/PufferAI/PufferLib/blob/35f165c4f721992022f4962708ce0cbec1fdf8b1/pufferlib/config/default.ini#L28); [tweet from the author, Joseph Suarez](https://x.com/jsuarez5341/status/1972364875990229065)).

[^3]: See also GRPO's precursor [RLOO](https://arxiv.org/abs/2402.14740v2), and [this interesting discussion on RL for LLM post-training](https://lancelqf.github.io/note/llm_post_training/).

[^4]: Note that there is no _unbiased_ reparameterization of _categorical_ sampling, i.e. sampling from a discrete distribution. The [straight-through](https://arxiv.org/abs/1308.3432), [Gumbel-Softmax](https://arxiv.org/abs/1611.01144)/[concrete](https://arxiv.org/abs/1611.00712) and [ReinMax](https://github.com/microsoft/ReinMax) estimators are all biased, though each of these improves upon the previous one.

[^5]: Assuming a differentiable simulator, rather than a learned model of the environment dynamics. In the latter case, we would still get _unbiased_ gradients, but they would be derived from an objective that is itself biased, as is the case when backpropagating through a learned critic.
