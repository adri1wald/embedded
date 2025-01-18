# Chip

Board: NRF52840-DK

Chip: Cortex-M4 32-bit processor with FPU

Data sheet: [Local PDF](./datasheet.pdf)
Board documentation: [Local PDF](./board.pdf)

# Setup

## Install target toolchain

```bash
rustup target add thumbv7em-none-eabihf
```

## Install and setup `cortex-m-rt`

Flash:

- Start: 0x00000000 <!-- Source: ./datasheet:23 of the data sheet -->
- Length: 1024K <!-- Source: ./datasheet:2 of the data sheet -->

RAM: 256K

- Start: 0x20000000 <!-- Source: ./datasheet:23 of the data sheet -->
- Length: 256K <!-- Source: ./datasheet:2 of the data sheet -->

```bash
cargo add cortex-m-rt
```

Provide a `memory.x` file containing data about the memory layout of the chip:

```bash
cat > memory.x <<EOF
MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH = 1024K
    RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}
EOF
```

## Install and setup `probe-rs`

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

Set the chip type:

```bash
cat > Embed.toml <<EOF
[default.general]
chip = "nRF52840_xxAA"
EOF
```

Flash the chip:

```bash
cargo embed
```

# Talk to me, goose!

```bash
cargo add rtt-target 
# https://docs.rs/rtt-target/latest/rtt_target/
#
# The printing macros require a critical section which is platform-dependent.
cargo add cortex-m --features critical-section-single-core
```

```rust
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    // Initialize the RTT target
    rtt_init_print!();

    // Print something
    rprintln!("Talk to me, goose!");

    // Loop forevermore
    loop {}
}

```

# Shine bright like a diamond

## What connects to the LEDs?

<!-- N.B. this information is _not_ found in the datasheet because it covers the nRF52840 chip, not the nRF52840-DK board -->

<!-- Source: ./board.pdf:5 -->

- LED1 (green) = P0.13
- LED2 (green) = P0.14
- LED3 (green) = P0.15
- LED4 (green) = P0.16

## GPIO registers

<!--
- Look for GPIO section in table of contents
- Found 6.9 GPIO - General purpose input/output at ./datasheet:322
    - Registers at ./datasheet:324
    - Pin assigments at ./datasheet:926
-->

<!-- Source: ./datasheet:324 -->

GPIO
- Base address: 0x50000000
- Description: General purpose input and output

P0
- Base address: 0x50000000
- Description: General purpose input and output, port 0
- Ports: P0.00 to P0.31

P1
- Base address: 0x50000300
- Description: General purpose input and output, port 1
- Ports: P1.00 to P1.15

## What we need to do?

TODO

<!-- See ./datasheet:325 for offsets -->

# Don't push me

## WHere are the buttons?

Push buttons
- BUTTON1 = SW1 = P0.11
- BUTTON2 = SW2 = P0.12
- BUTTON3 = SW3 = P0.24
- BUTTON4 = SW4 = P0.25
- BOOT = SW5 = boot/reset