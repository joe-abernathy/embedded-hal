# rust-embedded-hal

This repository aims to develop a Hardware Abstraction Layer (HAL) for embedded systems from scratch using Rust. This is primarily intended as a personal learning experience to enhance my embedded programming skills with Rust. This project will initially target the RP2040 microcontroller used in several Raspberry Pi boards, but may be extended as development continues. I plan to create a separate HAL project in parallel with this one, using C, to help build my embedded C skills and to gain a better understanding of the differences between the two languages in the embedded environment.

Goals:
- Develop a functional HAL using Rust
- Improve my understanding of low-level programming, embedded systems, and hardware interactions
- Collaborate with the community to learn from experienced embedded developers and receive feedback on my code
- Try to keep from pulling my hair out when I realize I'm in over my head

Please Note:
- This project is in its very early stages and may contain bugs, incomplete features, or suboptimal implementation. Any suggestions are welcome
- My primary goal with this project is personal growth. This codebase may not reflect production-ready quality
- I'm relatively new to Rust, so please bear with me as I learn. Feedback and guidance are always welcome!


**Progress to this point:**
- Generated a Peripheral Access Crate (PAC) for the RP2040 board from an SVD file provided by Raspberry Pi (found [here](https://github.com/raspberrypi/pico-sdk/blob/6a7db34ff63345a7badec79ebea3aaef1712f374/src/rp2040/hardware_regs/rp2040.svd])) using svd2rust.
- 
