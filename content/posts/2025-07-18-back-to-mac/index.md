+++
title = "I Used Arch, BTW: Rediscovering macOS"
draft = true
+++

_...and setting it up for neuro-AI research and engineering._

TL;DR: After 9 years of running Arch Linux (on non-Apple hardware), I bought a M4 Pro MacBook Pro to use as a daily driver.
This recounts my experience configuring this new machine to hit the ground running as quickly as possible with a productive setup to support my research and other endeavors.
If you're a Linux user making the switch to Apple Silicon, or thinking of doing so, this post may be of interest.

<!--  After 9 Years on Arch Linux -->
<!--
TL;DR: Local Arch Linux user begrudgingly returns to macOS.

This is a story of betrayal, acceptance, and/or nostalgia. -->

## Some Context

As of writing, I am midway through my PhD at McGill University and [Mila](https://mila.quebec/en/directory/yohai-eliel-berreby), focusing on neuro-AI research at [McGill's Department of Physiology](https://www.mcgill.ca/physiology/).
My prior background is computer engineering, which I studied at [Télécom Paris](https://en.wikipedia.org/wiki/T%C3%A9l%C3%A9com_Paris).

My workflow as a PhD student involves a mix of, non-comprehensively:
- Triaging, reading and annotating papers using [Zotero](https://www.zotero.org/);
- Writing code in [Zed](https://zed.dev/);
- Managing Python projects using Astral's [uv](https://docs.astral.sh/uv/) to make dependency management a breeze;
- Writing bespoke command-line utilities in [Rust](https://www.rust-lang.org/);
- Using [JAX](https://github.com/jax-ml/jax) and [PyTorch](https://pytorch.org/) for numerical simulations and deep (reinforcement) learning;
- Doing exploratory development using local [Jupyter Lab](https://github.com/jupyterlab/jupyterlab) notebooks:
  - Most code lives in regular Python files and gets auto-reloaded using `%load_ext autoreload` and `%autoreload 2`.
  - I frequently make interactive matplotlib plots using
  [ipympl](https://github.com/matplotlib/ipympl) and [ipywidgets](https://ipywidgets.readthedocs.io/en/stable/).
  - All notebooks are checked into git in text format using [jupytext](https://github.com/mwouts/jupytext) pairing.
- Firing off longer-running jobs on remote compute resources, using bespoke compute nodes or [SLURM](https://slurm.schedmd.com/overview.html) clusters;
- Writing down notes in my [org-roam](https://github.com/org-roam/org-roam/) personal knowledge base;
- Writing papers and slides in [Typst](https://github.com/typst/typst) or LaTeX (if I have to);
- Collaborating with colleagues on [Zulip](https://zulip.com/), Slack, or in person;
- Attending scientific meetings in-person or virtually (and taking live notes in org-roam);
- Staring intensely at a whiteboard/wall/ceiling when I'm mulling over a problem;
- Refining my personal setup;
- Debugging anything required, ranging from driver issues on compute workstations, to sub-millisecond timing code in experimental setups for [psychophysics](https://en.wikipedia.org/wiki/Psychophysics) using custom-built photodiode-based contraptions based on [Teensy](https://www.pjrc.com/teensy/) microcontrollers, to shared object loading in [proprietary software for high-speed cameras](https://flir.custhelp.com/app/answers/detail/a_id/4327/~/flir-spinnaker-sdk---getting-started-with-the-spinnaker-sdk), to performance or correctness issues in third-party code, etc;
- Writing blog posts such as this one using a static site generator, [Zola](https://github.com/getzola/zola).

Depending on the needs of the hour, I'll put on my researcher, engineer, sysadmin, or communicator hat, and I need my computing environment to support this workflow.

## Nine Beautiful Years of Arch Linux

I grew up on Mac OS 9, then Mac OS X.
While the UNIX backbone of Mac OS X made it a great introduction to computing, it ultimately proved unacceptably limiting for my purposes.
As a result, from 2015/2016 to 2025, I've faithfully used Arch Linux as my daily driver. I've run Arch on a trusty [Asus Zenbook UX410U](https://www.asus.com/laptops/for-home/zenbook/zenbook-ux410/), two custom-built desktops, many VMs, two consecutive Tuxedo Pulse 15 laptops from [Tuxedo Computers](https://www.tuxedocomputers.com/), an Asus TUF A15, and a few other devices.

From day one, I embraced the Arch spirit, consistently running either [i3](https://i3wm.org/) or [sway](https://swaywm.org/) as my tiling window managers of choice, complete with a panel of shortcuts allowing me to get much of my computer use done at the speed of thought.

Over the years, a number of friends and coworkers have adopted similar setups, in search of a streamlined yet ruthlessly efficient system configuration.

Arch is an _excellent_ distro.
This whole time, it's been difficult for me to imagine using anything else.
Occasionally having to use a Mac or Windows PC seemed like a test of patience; Arch felt like _home_.
A computing environment that I trust, that I understand, built by and for me, sculpted according to my precise needs, changing by my will and not due to some tech company's idea of what's good for me ("hey, it's 2025, we should [make everything unreadable!](https://www.apple.com/ca/newsroom/2025/06/apple-introduces-a-delightful-and-elegant-new-software-design/)").

Damn, I'm getting nostalgic just writing this.

## Giving Up

Given the above, it might come as a surprise that a few weeks ago, I caved in and bought a [14" M4 Pro MacBook Pro](https://www.apple.com/ca/shop/buy-mac/macbook-pro/14-inch-m4), which I intend to keep it running macOS.
Would running Linux on it be an option? I certainly considered it. Unfortunately, the excellent [Asahi Linux](https://asahilinux.org/) project only appears to deliver good support up until M2 chips, with [support for M4 Pro remaining entirely inadequate](https://asahilinux.org/docs/platform/feature-support/m4/).

This decision was shocking to nearly everyone around me. Some wondered if I was OK. As Glass Animals might put it: [🎵 whathehellishappening? 🎶](https://www.youtube.com/watch?v=wB3xTpw6ggI)

## I Swear I Can Explain

This fork in the road is hardware-motivated.
It comes after dealing with a series of catastrophic hardware failures, including a busted laptop hinge  [^1] and busted screw mounts, random boot failures, spotty Bluetooth, trackpack glitches, and more, across my last few devices.
The last straw was seeing two of my recent laptops kernel panic from being _held wrong_, and the latest one pop open like a piñata when opened without several layers of super-strength tape holding the case together. This machine is not even a year old! [^2]

After looking at a variety of options, including the obvious Lenovo ThinkPad line and the Framework, I decided to give Apple Silicon a chance this time around. This was partially out of curiosity. It is also because I am tired of fighting my hardware,  opting to fight "my" (Apple's, really) software this time around. Also, there's AppleCare+ and killer battery life. Yes, I am trying to feel better about it.

It's not every day that one rebuilds their workflow from scratch, much less on a new, relatively unfamiliar platform.
Thus, I figured I'd document my experience setting up this new machine to go from "brand new system" to "productive for my needs" in a few hours.

(embracing the power of sane defaults)

## Setting It Up

I decided to go for a _comfortable_ configuration: 48GB of Unified Memory, enough to run inference for various quantized LLMs and run small-scale ML training jobs ([within the bounds of lackluster `jax-metal` support](https://github.com/jax-ml/jax/issues?q=is%3Aopen+is%3Aissue+label%3A%22Apple+GPU+%28Metal%29+plugin%22)).

The macOS package management story is not great. There's no native solution, just the third-party-but-widely-used [Homebrew](https://brew.sh/) and [MacPorts](https://www.macports.org/), with neither being comparable to Arch's `pacman`.

Ah, wait, [you can also run Nix on macOS](https://blog.6nok.org/how-i-use-nix-on-macos/), and there's a [popular starter template](https://github.com/dustinlyons/nixos-config).

*TODO: pkgman story*


## A Selection of Essential Software

With the package manager out of the way, the next question becomes what software is essential for me to work.

Some of my day-to-day essentials are trivial to port over:
- **A text editor**. For me, it's currently [Zed](https://zed.dev/) with vim keybindings.
- **A proper shell**. `zsh` is the obvious and native choice, with [Starship](https://starship.rs/) as a fast, featureful prompt with sensible defaults.
- **A Python environment**. I'm doing ML research, so this is hardly negotiable. Thankfully, all the pain here is abstracted away by the excellent [`uv`](https://docs.astral.sh/uv/), which I can hardly praise enough for making Python development fun again.
- **Emacs**. Specifically, [Doom Emacs](https://github.com/doomemacs/doomemacs). Yes, I'm listing it separately from the text editor. These days, I'm only using Emacs for [magit](https://magit.vc/) and [org-roam](https://github.com/org-roam/org-roam/).
- **A web browser**. Firefox.
- **Spotify**.
- [**Zotero**](https://www.zotero.org/), for academic citation management.


For things that are more integrated with the OS, it's time to see what the macOS ecosystem has to offer:
- [Raycast](https://www.raycast.com/) is a widely-acclaimed launcher.
- [skhd.zig](https://github.com/jackielii/skhd.zig)



Remapped Caps Lock to Escape: https://vim.fandom.com/wiki/Map_caps_lock_to_escape_in_macOS

Zed as a cask, through nix homebrew as supported by my starter tpl

ProtonMail app
Firefox
TailScale GUI

!!! I had the GID issue and had to set stateVersion = 5 in the starter config (TODO mention the exact commit used - da88287) - also had to COMMENT OUT the nodejs stuff -> open issue?


---

Essential parts of my workflow that I immediately realize I miss:
- fast window switching - typing a few chars and immediately getting the window
- having new shells open in the same working dir as the last command
-

---

TailScale + ssh is great.
I can also do TailScale + syncthing for core files. key annoyance can be binary files from compilation caches etc

--

Firefox extensions:
- uBlock Origin
- Zotero Connector

--

KeePass situation? KeePassXC etc... restore syncthing sync for this one?

--

what about proton drive sync?

---

Moving to nix is a big step...

--

AeroSpace: 4.3. A note on ‘Displays have separate Spaces’ -> i disabled that, and set AeroSpace to launch on startup

SyncThing -> starting at login

added Routine to casks

---

got "unused config key: class" on alacritty w/ default config
then unused 'decorations'
and also buggy stuck emacs

---

https://docs.anthropic.com/en/docs/claude-code/troubleshooting#alternative-solution%3A-create-a-user-writable-npm-prefix-for-global-installs

-> eventually just decided to use the "claude-code" nix pkg

---

raycast hotkeys + aliases are amazing. not installing skhd for now
https://manual.raycast.com/command-aliases-and-hotkeys

---

Things I like:
- Beautiful font rendering
- Even the Weather app looks amazing


---

excited for the Tahoe apple neural engine FoundationModel API...


[^1]: Look up "Asus TUF hinge" online and witness the countless complaints.
[^2]: It is still under warranty, but it is in such a state that looking at it wrong will finish it off. Given that Asus is notorious for failing to honor their warranty and that I don't expect underpaid support engineers to treat it with the necessary deference, I am giving this cursed machine an early retirement as an unmoving appliance. It'll be fine; just don't breathe near it.
