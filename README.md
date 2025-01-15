# Chip

Board: NRF52840-DK

Chip: Cortex-M4 32-bit processor with FPU

Data sheet: [Local PDF](./datasheet.pdf)


# Setup

## Install target toolchain

```bash
rustup target add thumbv7em-none-eabihf
```

## Install and setup `cortex-m-rt`

Flash:

- Start: 0x00000000 <!-- Source: page 23 of the data sheet -->
- Length: 1024K     <!-- Source: page 2 of the data sheet -->

RAM: 256K

- Start: 0x20000000 <!-- Source: page 23 of the data sheet -->
- Length: 256K      <!-- Source: page 2 of the data sheet -->

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
