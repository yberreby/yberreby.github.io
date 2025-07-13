+++
title = "Back to macOS After 9 Years on Arch Linux"
draft = true
+++

I grew up on Mac OS 9 and Mac OS X.
The UNIX backbone of Mac OS X made it a great introduction to computing, but it ultimately proved unacceptably limiting for my purposes.
As a result, since 2015-2016, I've been a faithful (Arch) Linux user. I've run Arch on a trusty [Asus Zenbook UX410U](https://www.asus.com/laptops/for-home/zenbook/zenbook-ux410/), two custom-built desktops, many VMs, two consecutive Tuxedo Pulse 15 laptops from [Tuxedo Computers](https://www.tuxedocomputers.com/), an Asus TUF A15, and a few other devices.

From day one, I've embraced the Arch spirit, consistently running either [i3](https://i3wm.org/) or [sway](https://swaywm.org/) as my tiling window managers of choice, complete with a panel of shortcuts allowing me to get much of my computer use done at the speed of thought.

Over the years, a number of friends and coworkers have adopted similar setups, in search of a streamlined yet ruthlessly efficient system configuration.

Arch is an _excellent_ distro.
This whole time, it's been difficult for me to imagine using anything else.
Occasionally having to use a Mac or Windows PC seemed like a test of patience; Arch felt like _home_.
A computing environment that I trust, that I understand, built by and for me, sculpted according to my precise needs, changing by my will and not due to some tech company's idea of what's good for me ("hey, it's 2025, we should [make everything unreadable!](https://www.apple.com/ca/newsroom/2025/06/apple-introduces-a-delightful-and-elegant-new-software-design/)").

Damn, I'm getting nostalgic just writing this.

A few weeks ago, I caved in and bought a [14" M4 Pro MacBook Pro](https://www.apple.com/ca/shop/buy-mac/macbook-pro/14-inch-m4), which I intend to keep it running macOS.
Would running Linux on it be an option? I certainly considered it. Unfortunately, the excellent [Asahi Linux](https://asahilinux.org/) project only appears to deliver good support up until M2 chips, with [support for M4 Pro remaining entirely inadequate](https://asahilinux.org/docs/platform/feature-support/m4/).

This decision was shocking to nearly everyone around me.

As Glass Animals might put it: [🎵 whathehellishappening? 🎶](https://www.youtube.com/watch?v=wB3xTpw6ggI)

## I Swear I Can Explain

This fork in the road is hardware-motivated.
It comes after dealing with a series of catastrophic hardware failures, including a busted laptop hinge  [^1] and busted screw mounts, random boot failures, spotty Bluetooth, trackpack glitches, and more, across my last few devices.
The last straw was seeing two of my recent laptops kernel panic from being _held wrong_, and the latest one pop open like a piñata when opened without several layers of super-strength tape holding the case together. This machine is not even a year old! [^2]

After looking at a variety of options, including the obvious Lenovo ThinkPad line and the Framework, I decided to give Apple Silicon a chance this time around. This was partially out of curiosity. It is also because I am tired of fighting my hardware,  opting to fight "my" (Apple's, really) software this time around. Also, there's AppleCare+ and killer battery life. Yes, I am trying to feel better about it.

## Setting It Up

I decided to go for a _comfortable_ configuration: 48GB of Unified Memory, enough to run inference for various quantized LLMs and run small-scale ML training jobs ([within the bounds of lackluster `jax-metal` support](https://github.com/jax-ml/jax/issues?q=is%3Aopen+is%3Aissue+label%3A%22Apple+GPU+%28Metal%29+plugin%22)).

The macOS package management story is not great. There's no native solution, just the third-party-but-widely-used [Homebrew](https://brew.sh/) and [MacPorts](https://www.macports.org/), with neither being comparable to Arch's `pacman`

Ah, wait, [you can also run Nix on macOS](https://blog.6nok.org/how-i-use-nix-on-macos/), and there's a [popular starter template](https://github.com/dustinlyons/nixos-config).



[^1]: Look up "Asus TUF hinge" online and witness the countless complaints.
[^2]: It is still under warranty, but it is in such a state that looking at it wrong will finish it off. Given that Asus is notorious for failing to honor their warranty and that I don't expect underpaid support engineers to treat it with the necessary deference, I am giving this cursed machine an early retirement as an unmoving appliance. It'll be fine; just don't breathe near it.
