# Chip

Board: NRF52840-DK

Chip: Cortex-M4 32-bit processor with FPU

Data sheet: [Local PDF](./datasheet.pdf)


# Setup

## Install target toolchain

```bash
rustup target add thumbv7em-none-eabihf
```

## Install cortex-m-rt

```bash
cargo add cortex-m-rt
```

### Parameters

Flash:

- Start: 0x00000000 <!-- Source: page 23 of the data sheet -->
- Length: 1024K     <!-- Source: page 2 of the data sheet -->

RAM: 256K

- Start: 0x20000000 <!-- Source: page 23 of the data sheet -->
- Length: 256K      <!-- Source: page 2 of the data sheet -->
