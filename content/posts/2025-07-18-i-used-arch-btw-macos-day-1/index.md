+++
title = "I Used Arch, BTW: macOS, Day 1"
draft = false
+++

TL;DR:
I used Arch Linux for nine years as a daily driver on non-Apple laptops.
I received my new M4 Pro MacBook Pro yesterday.
This recounts my experience configuring it to hit the ground running from day 1.

If you're a Linux user making the switch to Apple Silicon, or thinking of doing so, this post may be of interest.

## What I Need My Computer For

The way one uses their computer is deeply colored by their needs, values, and habits.

As of writing, I am midway through my PhD at McGill University and [Mila](https://mila.quebec/en/directory/yohai-eliel-berreby), focusing on neuro-AI research at [McGill's Department of Physiology](https://www.mcgill.ca/physiology/).
My prior background is computer engineering, which I studied at [Télécom Paris](https://en.wikipedia.org/wiki/T%C3%A9l%C3%A9com_Paris).

My workflow as a PhD student involves a mix of:
- Filing, reading and annotating scientific papers using [Zotero](https://www.zotero.org/);
- Writing code in [Zed](https://zed.dev/), a fast text editor with first-class support for [LSP and Tree-Sitter](https://zed.dev/docs/configuring-languages), GPU acceleration, [Rust-powered multithreading](https://zed.dev/blog/we-have-to-start-over), and, non-negotiably, [vim bindings](https://zed.dev/docs/vim);
- Doing most of my computer interaction inside of a [zsh](https://wiki.archlinux.org/title/Zsh) shell with a featureful prompt such as [Starship](https://starship.rs/) (previously, [grml-zsh-config](https://grml.org/zsh/) or [oh-my-zsh](https://ohmyz.sh/));
- Managing Python projects using Astral's [uv](https://docs.astral.sh/uv/) to make dependency management a breeze;
- Writing bespoke command-line utilities in [Rust](https://www.rust-lang.org/);
- Using [JAX](https://github.com/jax-ml/jax) and [PyTorch](https://pytorch.org/) for numerical simulations and deep (reinforcement) learning;
- Doing exploratory development using local [Jupyter Lab](https://github.com/jupyterlab/jupyterlab) notebooks[^1];
- Firing off longer-running jobs on remote compute resources, using on-premises workstations or larger [SLURM](https://slurm.schedmd.com/overview.html) clusters;
- Writing down notes in my [org-roam](https://github.com/org-roam/org-roam/) personal knowledge base;
- Writing papers and slides in [Typst](https://github.com/typst/typst) or LaTeX (if I have to);
- Collaborating with colleagues on [Zulip](https://zulip.com/), Slack, or in person;
- Attending scientific meetings in-person or virtually (and taking live notes in org-roam);
- Staring intensely at a whiteboard/wall/ceiling when I'm mulling over a problem (no tech required);
- Debugging anything and everything, ranging from driver issues on compute workstations, to sub-millisecond timing code in experimental setups for [psychophysics](https://en.wikipedia.org/wiki/Psychophysics) using custom-built photodiode-based contraptions based on [Teensy](https://www.pjrc.com/teensy/) microcontrollers, to shared object loading in [proprietary software for high-speed cameras](https://flir.custhelp.com/app/answers/detail/a_id/4327/~/flir-spinnaker-sdk---getting-started-with-the-spinnaker-sdk);
- Writing blog posts such as this one using a static site generator, [Zola](https://github.com/getzola/zola).

Depending on the needs of the hour, I'll put on my researcher, engineer, sysadmin, or communicator hat, and I need my computing environment to support this workflow.

For the longest time, this role was fulfilled by my Arch Linux laptops and desktops.

## Nine Beautiful Years of Arch Linux

I didn't start out on Linux.
I digitally grew up on [System 7](https://en.wikipedia.org/wiki/System_7), [Mac OS 9](https://en.wikipedia.org/wiki/Mac_OS_9), then [Mac OS X](https://en.wikipedia.org/wiki/Mac_OS_X_Snow_Leopard), in an Apple-centric household where "Windows" was a profanity.
While the UNIX backbone of Mac OS X made it a great introduction to computing, I eventually grew frustrated with the developer experience, exorbitant hardware repair costs (what do you mean, everything is glued or soldered?), lackluster GPU performance, and poor support for software projects that I wanted to hack around with.
As a result, I switched to [Arch Linux](https://archlinux.org/) in 2016 after briefly trying Debian, Ubuntu, and Fedora
Since then, I've faithfully used Arch as my daily driver, running it on a trusty [Asus Zenbook UX410U](https://www.asus.com/laptops/for-home/zenbook/zenbook-ux410/), two desktop workstations that I built from individual components, many VirtualBox VMs, two consecutive Tuxedo Pulse 15 laptops from [Tuxedo Computers](https://www.tuxedocomputers.com/), an Asus TUF A15, and a few other devices. I reinstalled Arch a couple of times at the beginning, then mostly migrated my systems one SSD swap or [`dd`](https://wiki.archlinux.org/title/Dd) clone at a time after the thrill of a fresh install wore off.


From day one, I embraced the Arch spirit, consistently running either [i3](https://i3wm.org/) or [sway](https://swaywm.org/) as my tiling window managers of choice, complete with a panel of shortcuts allowing me to get much of my computer use done at the speed of thought. I'm proud to say that over the years, a number of friends and coworkers adopted similar setups after seeing how fast one's computer use could get, in search of a streamlined yet ruthlessly efficient system configuration.

Arch is a stellar distro.
The package management story is top-notch, with up-to-date packages that are easy to create and maintain, and generally come very close to the upstream sources, without a slew of distro-specific patches.
[`pacman`](https://wiki.archlinux.org/title/Pacman) just works, and [AUR](https://aur.archlinux.org/) gives you access to the most obscure of packages.
The documentation on the [Arch Wiki](https://wiki.archlinux.org/title/Main_page) is excellent, to the point that it's often worth checking when troubleshooting a problem on another distro.
On Arch, you build your system piece by piece, trading initial setup time for deep understanding.
This is both deeply intellectually satisfying and practically empowering.

Given how freeing the Arch experience is, it has been difficult for me to imagine using anything else.
Occasionally having to use a Mac or Windows PC seemed like a test of patience; Arch felt like _home_.
A computing environment that I trust, that I understand, built by and for me, sculpted according to my precise needs, changing by my will and not due to some tech company's idea of what's good for me ("hey, it's 2025, we should [make everything unreadable!](https://www.apple.com/ca/newsroom/2025/06/apple-introduces-a-delightful-and-elegant-new-software-design/)").

So... what's the catch, then?
Why migrate away from something so good?

## Hardware That Wears You Out

While Arch Linux itself, as a Linux distribution, is great, it needs to run on _something_. And when it comes to laptops, that _something_ is not great.

PC laptops are, mostly, not built for Linux. This means that any laptop purchase has historically come with a series of driver-related caveats.
Will suspend-to-RAM (sleep) work? How is the power management story --- will I get more than a few hours of battery life after I install [TLP](https://wiki.archlinux.org/title/TLP)? Do the GPU drivers work well / at all, including with [PRIME](https://wiki.archlinux.org/title/PRIME) / [Optimus](https://wiki.archlinux.org/title/NVIDIA_Optimus) iGPU-dGPU switching? Will my Wi-Fi chipset randomly disconnect me from WPA2 Enterprise networks, or straight up cause a kernel panic?
Can I use any of the "fancy" manufacturer features, such as a fingerprint reader, or are they locked behind proprietary drivers that only support Windows?
Will my Bluetooth randomly disconnect and reconnect, or drop packets excessively?

Most of these issues are not really a concern on desktop workstation / server hardware, and indeed, I wouldn't run anything _but_ Linux on those. However, on a laptop, having to take all of these into account is exhausting.
I don't know about you, but I want to _use my machine to tackle important issues_, not to have my laptop _be_ the issue.

Driver issues aren't all there is.
Anecdotally, I've recently been dealing with a series of catastrophic hardware failures, including a busted laptop hinge[^2] and busted screw mounts, random boot failures, spotty Bluetooth, trackpad glitches, and more, across my last few devices.
Recently, I saw two of my laptops kernel panic from being _held wrong_, and the latest one pop open like a piñata when opened without several layers of super-strength tape holding the case together. This machine is not even a year old![^3]

Now, of course, similar issues can and do occur on MacBooks as well, and my sample size is statistically insignificant.
However, I don't think it is too much to ask for 1) warranties that are actually honored, and 2) hardware that amounts to more than a haphazardly assembled pile of fast components.

Could a [Dell XPS](https://wiki.archlinux.org/title/Dell_XPS_15), [Lenovo ThinkPad](https://wiki.archlinux.org/title/Laptop/Lenovo), [Surface Pro](https://wiki.archlinux.org/title/Microsoft_Surface_Pro_9), [TUXEDO](https://www.tuxedocomputers.com/en/Arch-Linux-and-Manjaro-on-TUXEDO-computers.tuxedo) or [Framework Laptop](https://frame.work/ca/en) fit the bill? Probably, depending on the compromises one is willing to make.
However, I decided to give Apple Silicon a chance this time around. This was partially out of curiosity; Apple laptops are notorious for their combination of build quality, battery life, unified memory, and customer service. I had also briefly used a M1 MacBook Air out of necessity, while working on cross-compilation of OCaml code to iOS at [Routine](https://www.ycombinator.com/companies/routine), and came out quite impressed with the smoothness of the UX on this relatively cheap machine.

Thus, tired of fighting my _hardware_, I opted to fight "my" (Apple's, really) _software_ this time around, trading my meticulously-honed setup running for the foreign and locked-down macOS.


## Eating the Forbidden Fruit

When I caved in and bought a custom [14" 48GB M4 Pro MacBook Pro](https://www.apple.com/ca/shop/buy-mac/macbook-pro/14-inch-m4) with the intent of keeping macOS on it, I was immediately met with a mix of confusion (_this guy bought a Mac?!_), glee (_how the mighty have fallen!_), disappointment (_you should have bought a Framework, you fool_), acceptance (_congratulations on giving up!_), and concern (_are you sure you're OK?_). The general sentiment was [whathehellishappening? 🎶](https://www.youtube.com/watch?v=wB3xTpw6ggI).

Would running Linux on it be an option? No.
Unfortunately, [Asahi Linux](https://asahilinux.org/) only appears to deliver good support up until M2 chips, with [M4 Pro support remaining entirely inadequate](https://asahilinux.org/docs/platform/feature-support/m4/).
So, macOS it is.
Quite frankly, I probably still wouldn't try running Linux on it even if there were some Asahi support; if I'm going to be using Mac hardware, I might as well take full advantage of it.

## The Lazy Frankenmac

Running macOS does _not_ mean giving up on the past decade of accumulated experience.
I live inside of a shell, I want my configuration to be as declarative as possible, a good [tiling window manager](https://en.wikipedia.org/wiki/Tiling_window_manager) is non-negotiable, I want my window switching to be nearly instantaneous, and I am not about to start preferring the mouse/touchpad over the keyboard.

I got ahold of my new machine yesterday, and set out to set it up the "lazy power user" way: I wanted to opportunistically get back as much of my workflow as possible, without going down a rabbit hole of configuration.
Not all battles are worth fighting. It is a delicate balance between _"I don't have time to configure this, I have urgent things to do"_ and _"I am now taking three times as long to do anything because my SSH agent doesn't work, my most-frequently-used shortcuts no longer work, and I manage my packages with `./configure && make && make install`."_

I will live without my terminal's colors being configured _just right_ for now, as long as they're reasonable. Same with my shell prompt; if it has a decent git indicator and autocompletion, that's a good start. There is something freeing in embracing sane defaults, then gradually improving them.

Let's call this a Lazy Frankenmac; not in the Hackintosh sense, but in the sense that it's macOS set up in a Linux spirit.

What key ingredients would that involve?
- A sensible package and configuration management story.
- A modern shell environment.
- A tiling window manager, in the spirit of [`i3`](https://i3wm.org/) / [`sway`](https://swaywm.org/) / [`hyprland`](https://hypr.land/).
- A launcher in the spirit of [`dmenu`](https://wiki.archlinux.org/title/Dmenu) / [`rofi`](https://github.com/davatorium/rofi) / [`wofi`](https://github.com/SimplyCEO/wofi).

We're not going to get everything right in a day, far from it, but let's see how far we can get.

## [Nix](https://nixos.org/) as a Package Manager

First, let's get our package manager set up.
Installing GUI apps and CLI programs by dragging and dropping .app files or running installers manually is... not ideal, and neither is the locked-down Mac App Store.

The macOS package management story is not great. There's no good built-in solution.
[Homebrew](https://brew.sh/) is by far the most popular package manager, widely used by developers.
[MacPorts](https://www.macports.org/) is older but less popular.
Neither is comparable to Arch's `pacman`.

Homebrew isn't that bad.
Its filesystem permission handling is controversial, to say the least, and in my prior experience, it has a tendency to fail at a package manager's main job --- namely, ensuring that installing a new dependency doesn't break the system.
Those issues aside, it mostly works.

There's something more interesting to try, though: you can run [Nix](https://nixos.org/) on macOS, with [`nix-darwin`](https://github.com/nix-darwin/nix-darwin) doing the legwork and [`nix-homebrew`](https://github.com/zhaofengli/nix-homebrew) serving as a convenient escape hatch for when you need Homebrew [Casks](https://github.com/Homebrew/homebrew-cask) or when the Nix package you need is missing or outdated.

Nix is conceptually appealing (purely functional, declarative package and configuration management, yay!), but can be quite difficult to troubleshoot.
I've used it a few times in limited contexts, but never as my primary package manager.
Thankfully, there's a [popular starter template](https://github.com/dustinlyons/nixos-config) that puts together `nix-darwin`, `nix-homebrew` and various other goodies to quickly get started with Nix on macOS!


## Of Course It Doesn't Work Out of The Box

Following @dustinlyons's instructions as of [commit `da88287`](https://github.com/dustinlyons/nixos-config/tree/da88287), I installed _Nix_ using the Determinate Systems installer, not to be confused with installing _Determinate Nix_ using the Determinate Systems installer. Rookie mistake! Definitely not a footgun.

I then _did not_ add `experimental-features = nix-command flakes`  to `/etc/nix/nix.conf` during setup. Those were already in `extra-experimental-features`.

I followed the `starter-with-secrets` template:

```zsh
mkdir -p nixos-config
cd nixos-config
nix flake \
    --extra-experimental-features 'nix-command flakes' \
    init \
    -t github:dustinlyons/nixos-config#starter-with-secrets
```

That `--extra-experimental-features` flag, as specified by the template's README, was probably not necessary since it was already in `nix.conf`.

My MBP is running macOS Sequoia 15.5. This is a fresh install, so I shouldn't have to ["prepare Nix" for a Sequoia installation](https://determinate.systems/posts/nix-support-for-macos-sequoia/). Yet, the installation failed at first, with a `mismatching GID` error.
[Changing `stateVersion = 4;` to `stateVersion = 5;`](https://github.com/nix-darwin/nix-darwin/issues/1339#issuecomment-2661471788) in `hosts/darwin/default.nix` resolved the issue.

I then encountered more issues.

- `test-fs-cp.mjs` failed when trying to build `nodejs-22.17.0`. I don't need this LTS node version anyway, so I replaced the `nodejs` package with `nodejs_24` and removed the corresponding `nodePackages` entries from the relevant `packages.nix` config, and the build succeeded.
- The `alacritty` configuration included in the template was obsolete, resulting in a number of `unused config key` warnings.
- I had an issue with the `masApps` key, which included entries for `1password` and `wireguard`. I discarded these, as I need neither.
- The Dock configuration was slightly broken out-of-the-box, but easy to adjust.
- The bundled Emacs configuration seems broken, with Emacs freezing on startup.

Additionally, installing the [AeroSpace](https://github.com/nikitabobko/AeroSpace) tiling WM as a Cask required adding an entry to `flake.nix`'s inputs:

```
homebrew-aerospace = {
  url = "github:nikitabobko/homebrew-tap";
  flake = false;
};
```

## Once Nix is Set Up, You're Flying

Once I got `nix run .#build-switch` to work well enough, it was a breeze to install a number of tools as a combination of `nix-homebrew` Casks, Nix packages, and `home-manager` entries.

In no particular order, these include Zed, Zotero, Firefox, Spotify, Proton Mail, Proton Mail Bridge, [`claude-code`](https://mynixos.com/nixpkgs/package/claude-code), [`rustup`](https://nixos.wiki/wiki/Rust), [`uv`](https://mynixos.com/nixpkgs/package/uv), [`zola`](https://mynixos.com/nixpkgs/package/zola), [`typst`](https://mynixos.com/nixpkgs/package/typst), Signal, WhatsApp Web, Slack, Alacritty, KeePassXC, Raycast, Claude Desktop, Zulip.
I did install [Tailscale](https://tailscale.com/) through its official `.pkg` installer; it needs a lot of low-level system access, and debugging the macOS networking stack is not my top priority at the moment.

Key features are OK:

**Window Management**: For now, [AeroSpace](https://github.com/nikitabobko/AeroSpace) + [Raycast](https://www.raycast.com/) are a very acceptable replacement for sway + rofi/wofi.
Following the official AeroSpace recommendation, I disabled the `Displays have separate Spaces` macOS feature.
I have been enjoying [Raycast's hotkeys and aliases](https://manual.raycast.com/command-aliases-and-hotkeys) so far; no [`skhd`](https://github.com/jackielii/skhd.zig) required (yet?).

**Text Edition**: I quickly [remapped Caps Lock to Escape](https://vim.fandom.com/wiki/Map_caps_lock_to_escape_in_macOS) and set Zed to use vim bindings.
@dustinlyons's starter template includes a reasonable configuration for vim itself.

**Python and Rust**: `uv` works, and I'm able to build Rust projects with dependencies on e.g. `openssl` using [`direnv`](https://direnv.net/)'s `use flake` and a lightweight `flake.nix`.

**Browser**: Firefox. Not much to say here. I only needed two Firefox extensions from the get-go: [Zotero Connector](https://www.zotero.org/download/connectors) to save papers, uBlock Origin to get rid of ads.
I could probably even have done without the latter, given Firefox's new built-in privacy features.

**Shell**: The zsh configuration included in the template is a decent start. I added [`atuin`](https://atuin.sh/) through `home-manager`, for featureful Ctrl+R shell history search.

**Terminal Emulator**: Currently using [alacritty](https://github.com/alacritty/alacritty). Also considering [WezTerm](https://wezterm.org/index.html).

## Closing Words

This is how far I got on the first day.
This setup is far from perfect, but it's already usable; I'm able to perform most steps of my workflow without pulling my hair out.

I'll be honing the system gradually.
This includes slimming down and modernizing my `nixos-config`, understanding Nix beyond the surface level, using AeroSpace and Raycast to their fullest potential, and straightening out various little annoyances as they pop up.

I'm surprised by how much I like this machine already.
It is great to look at, hold, and type on.
The trackpad is top-notch.
Using it generally feels snappy.
The built-in software gives the general impression of having been designed, not cobbled together.

It remains to be seen how these initial impressions will hold up over time, and it is certainly a shame that one has to resort to third-party tools for something as fundamental as package management.

But I suppose I can live with it in exchange for a machine that doesn't kernel panic when I open the lid at the wrong angle.

[^1]: Most of the code used in my notebooks lives in regular Python files and gets auto-reloaded using `%load_ext autoreload` and `%autoreload 2`. I frequently make interactive matplotlib plots using [ipympl](https://github.com/matplotlib/ipympl) and [ipywidgets](https://ipywidgets.readthedocs.io/en/stable/). All notebooks are checked into git in text format using [jupytext](https://github.com/mwouts/jupytext) pairing.
[^2]: Look up "Asus TUF hinge" online and witness the countless complaints.
[^3]: It is still under warranty, but it is in such a state that looking at it wrong will finish it off. Given that Asus is notorious for failing to honor their warranty and that I don't expect underpaid support engineers to treat it with the necessary deference, I am giving this cursed machine an early retirement as an unmoving appliance. It'll be fine; just don't breathe near it.
