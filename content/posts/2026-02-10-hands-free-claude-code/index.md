+++
title = "Hands-Free Claude Code with the Agent SDK"
date = "2026-02-10"
+++


I'm a decently heavy Claude Code user.
Running some usage stats by data mining `~/.claude` and running [`ccusage`](https://github.com/ryoppippi/ccusage),
it turns out that I average a few thousand messages per day,
and I ran through 2.72 billion tokens 
(2.63B of those being cache reads, thankfully)
over the past 42 days.
If I weren't using a Max plan,
this would amount to $1822 in API costs.

What Karpathy calls [agentic engineering](https://x.com/karpathy/status/2019137879310836075) has become an absolutely essential part of my workflow.
For anything serious,
[code quality must not be compromised](https://heidenstedt.org/posts/2026/how-to-effectively-write-quality-code-with-ai/)
by the use of agentic AI;
sadly, the concepts of technical debt, brittle code and inadequate architecture
do not cease to exist just because a frontier LLM is writing the code.

That being said,
there is still a place for a looser approach
when exploring an idea,
getting a proof of concept up,
building a one-off piece of software,
or gathering information (to be checked carefully later).
In general,
it is also possible to let Claude run loose in `--dangerously-skip-permissions` mode
as long as you're keeping close track of what it's doing, and why, in real time.

For the longest time,
I've thought about integrating Claude Code with a local voice stack.
I had a shot at STT-TTS-LLM integrations years ago, but the tech wasn't up to the task at the time.
Now, however, the models (by which I mean, Claude Opus 4.6)
have gotten good enough to do an excellent job at inferring intent from noisy data,
which means that it's possible to get good results even with imperfect speech-to-text (STT).
Concurrently, STT models just keep getting better.
Seriously, try the [Voxtral-Mini-Realtime](https://huggingface.co/spaces/mistralai/Voxtral-Mini-Realtime) demo and report back [^1].

When I saw the new Voxtral models coming out and everyone going crazy about [Moltbot / OpenClaw](https://openclaw.ai/),
that was my cue to give a personal voice-enabled AI assistant another try.
So I started building what I call **Yad** (יד, "hand"):
a personal project with ears,
meant to _be my hands_ while I'm up on my feet
and away from the keyboard.

Yad has absolutely no pretense of being a product;
providing ongoing support (and security guarantees, which are _critical_ for a tool like this)
would be a commitment I cannot make.
For those who want plug-and-play solutions,
projects like OpenClaw appear much more relevant.
While I might gradually open-source components of Yad,
I am building it entirely for my own purpose and needs,
and this technical writeup is only meant to share my experience.

If you want something similar to what I describe here,
point your AI agent to this post's URL to get it started,
and start building!
Sculpting such a system to fit your precise needs and workflow is the point,
and building it is half the fun.
When I say it should be straightforward (proper async handling notwithstanding),
I really mean it;
I did not write a single line of code by hand in this project,
and got the first useful prototype working in just half a day with Opus 4.6.
The code can and will change daily as I iterate and the underlying tech changes.


## How it Works

I begin by starting Yad,
which is not a single process but a collection of largely-independent daemons,
communicating using Unix Domain Sockets (UDS) locally,
or TCP over my [Tailscale](https://github.com/tailscale/tailscale)
personal network.
The system was initially based on vanilla UDS,
but I am gradually incorporating [ZeroMQ](https://zeromq.org/)
in order to support event broadcasting with minimal complexity.

Each service is mostly responsible for one thing:
voice activity detection (VAD) with [TEN VAD](https://github.com/TEN-framework/ten-vad),
speech-to-text with [NVIDIA Parakeet TDT 0.6B v2](https://huggingface.co/nvidia/parakeet-tdt-0.6b-v2),
text-to-speech with [Pocket TTS](https://github.com/kyutai-labs/pocket-tts),
audio I/O with [CoreAudio AUHAL](https://developer.apple.com/library/archive/technotes/tn2091/_index.html) / [rodio](https://github.com/RustAudio/rodio),
AirPods event interaction flow based on [`AVAudioApplication.setInputMuteStateChangeHandler(_:)`](https://developer.apple.com/documentation/avfaudio/avaudioapplication/setinputmutestatechangehandler(_:)).

This last part—AirPods—is pretty important to the current design.
When I'm working with Yad active,
I put on my AirPods and act like I'm _on a call with Claude Code_.
When I want to do my own thing, I'm muted;
when I want to speak to it, I press the stem to unmute myself,
using `setInputMuteStateChangeHandler` (a flow inspired by [AirMute](https://github.com/Solarphlare/AirMute/))
to capture the relevant event.

I had initially used media player events (play/pause);
while it's easy to capture a Play event,
it was much trickier to detect a second stem click _while the AirPods' microphone was active_.
The AirMute-like approach solves that.
This setup makes interaction much more natural than using a wake word (saying "Hey Yad" every 10 seconds would get old _fast_),
and more reliable than relying solely on VAD, which would pick up background conversations when I don't want it to.

Feel free to swap out this AirPods-based flow with the hardware-software combination of your choice;
the important part is low-friction, reliable toggling between "interacting with the assistant"
and "keeping it dormant, but on standby".

Once I'm unmuted, the entire voice stack (VAD, STT, TTS) runs locally on my laptop (48GB M4 Pro MacBook Pro).
An audio segment, delimited by thresholding on TEN VAD scores,
gets captured and sent to Parakeet once I stop speaking or perform a second stem click.
It is transcribed within hundreds of milliseconds,
with what I've found to be very good transcription fidelity.

This transcribed text—or text that I input via the CLI, if I want to hear from Claude but not speak to it—gets sent to Opus 4.6
using a custom agentic loop built with the [Claude Agent SDK](https://platform.claude.com/docs/en/agent-sdk/overview).
This allows me to get tons of Claude Code's goodies—auto-compaction, standard and custom tools, sandboxing support, session resume, etc.—while retaining full control. I originally used a custom MCP server, but that makes the Claude Code instance be in control; the agent SDK allows my code to orchestrate Claude, not the other way around.
Critically, the Claude Agent SDK currently supports login with the Max plan,
meaning that I don't need to pay exorbitant API rates to run this,
and can just rely on my existing Claude subscription.
I have a feeling this might change in the future; I'm enjoying it for now.

Opus gets my input—which can come in at any time, allowing me to interject while it's already working on something in order to keep it from going off-rails—and responds to it following a voice-appropriate interaction format: frequent, short sentences, narrating what it's about to do, what information it has gathered, its reasoning for doing things.
These sentences get synthesized into audio with low latency by [Pocket TTS](https://kyutai.org/blog/2026-01-13-pocket-tts).
Opus's behavior and contextual understanding are guided by a combination of a custom system message,
and reminders / helpful metadata provided by my custom agent harness.
It is able, and encouraged, to ask me questions.
It maintains its own memory through a combination of a long-running continuously-compacted session,
and persistent Markdown files.
When it's busy thinking and running tool calls,
the harness plays a procedurally-generated keyboard clicking sound,
which I find to be a nice way to keep me engaged and fill waiting times
when I ask complex questions.

To make this useful, I gradually equip it with custom tools—written by and with Yad—which may range from simple information about what commands to run when, to `osascript`-based shell scripts to interact with the OS, to custom native programs using low-level system APIs.
I can have it read PDFs,
browse my [Zotero](https://www.zotero.org/) library of research papers,
look through my projects' code and git history,
perform web searches on my behalf,
and do all of the above by orchestrating subagents—courtesy of the Agent SDK.

Yesterday, I vocally guided it through connecting to my living room TV,
activating AirPlay via AppleScript,
and moving its own session window to it.
Today, I had it write a simple dashboard which listens to ZeroMQ events from the VAD service,
so that I get visual feedback on the voice stack and Claude's actions.
Streaming to a wide-screen TV makes it easy to get another view of what Claude is doing,
through a higher-bandwidth channel than voice alone, without having to get close to the computer.
The whole thing feels extremely fast, and will get faster as LLM inference does.

I think it's pretty cool. In some ways, it really feels like living in the future, and what Siri / Google Assistant should have been: a voice assistant that actually saves me time. Because no, I _didn't_ mean to set a timer. Damn it, Siri.

NB: No, this post was _not_ written by AI. I just like hyphens. Please don't take them away from me.

---

[To leave a comment, see the thread on Hacker News.](https://news.ycombinator.com/item?id=46966944)

[^1]: While I _wish_ I were using Voxtral-Mini-Realtime,
there was no real-time inference support for MLX / Apple Silicon when I checked.
Soon, soon.
