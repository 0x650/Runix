<img width="850" src="branding/RunixFull.png">

---

![Lines of code](https://img.shields.io/tokei/lines/github/0x650/runix?label=Lines%20Of%20Code&style=flat-square)
[![GitHub issues](https://img.shields.io/github/issues/0x650/runix?label=Issues&style=flat-square)](https://github.com/0x650/Runix/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/0x650/runix?label=Pull%20Requests&style=flat-square)](https://github.com/0x650/Runix/pulls)
[![GitHub](https://img.shields.io/github/license/0x650/runix?label=License&style=flat-square)](https://github.com/0x650/Runix/blob/master/LICENSE)
[![GitHub commit activity (branch)](https://img.shields.io/github/commit-activity/m/0x650/runix/master?label=Commit%20Activity&style=flat-square)](https://github.com/0x650/Runix/graphs/commit-activity)
[![GitHub Repo stars](https://img.shields.io/github/stars/0x650/runix?label=Stargazers&style=flat-square)](https://github.com/0x650/Runix/stargazers)

Runix is an experimental realtime UNIX-like OS written in the Rust programming language.  
It is under development, and is obviously not yet suitable for any kind of serious use.

## :question: What does "Runix" stand for?

It actually stands for two things!

- **R**ealtime **Unix**
- **Ru**st **Unix**

## :package: Building

Building can be done in Windows and macOS, but the makefile expects Linux to be able to generate an ISO.  
Thus, using Linux is recommended. Windows users may use WSL without a hitch.

1. Install **Rust**, **GNU Make**, and your distro's **build essentials**.
2. Execute `rustup target add x86_64-unknown-none`. This will download the required toolchain.
3. Run `make all` and wait for it to finish. It may take a while on the first build.

## :handshake: Contributing

Runix is open to pull requests and issue tickets! Any help is appreciated.  
A contribution guide will be added later into development.

# :balance_scale: License
This project is licensed under the **Apache License 2.0** which you can read [here](LICENSE).  
