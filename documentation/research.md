# How to hardware without a HAL 

1. **Find the Right Documentation**
   - **nRF52840 Product Specification**: This document from Nordic Semiconductor is your primary resource. It contains the memory map, peripheral descriptions, and register definitions. You will find GPIO (General-Purpose I/O) registers, their offsets, and the required bits to configure pins as outputs.
   - **DK Documentation**: Consult the Nordic documentation for the nRF52840-DK board itself. It will tell you which physical pins the LEDs are attached to (e.g., P0.13, P0.14, etc.). The DK documentation often includes a schematic or a quick reference for LED pin mappings.

2. **Identify the LED Pins**
   - Look at the nRF52840-DK documentation to confirm exactly which GPIO pins the onboard LEDs use.
   - Make sure you also note if the LED is active high or active low, because that affects how you write to the GPIO registers (on many Nordic DKs, the LEDs are active low, meaning driving the pin low turns the LED on).

3. **Review the Memory Map and Peripheral Base Address**
   - The Product Specification has a memory map table showing the base addresses for peripherals (for example, `GPIO` might be at a specific address like `0x50000000` or similar).
   - Under the GPIO or P0 peripheral section, find the register block layout. Each register (such as `OUT`, `DIR`, etc.) is described with its offset from the base.

4. **Learn the GPIO Registers**
   - **DIR Register**: Controls whether a pin is an input or an output. Typically you set a bit to `1` for output, `0` for input.  
   - **OUT Register**: Controls the pin state (logic high or low). Setting a bit to `1` might drive the pin high; setting it to `0` might drive it low.  
   - **PIN_CNF\[x\] Registers** (if available on this chip): Configure each pin’s drive strength, pull-up/pull-down, and input buffer. On the nRF52 series, each GPIO pin has a configuration register.  
   - Note any additional steps such as enabling the clock for the GPIO block if required. On some chips, the GPIO clock is enabled by default, but always verify in the “Clock Management” chapter.

5. **Follow the Steps to Blink the LED**
   - **Step A: Configure the pin**: By writing to the PIN_CNF\[x\] (or equivalent) register to set it as a push-pull output with no pull (or an appropriate drive mode). Then set the corresponding bit in the `DIR` register to mark it as an output.
   - **Step B: Turn it on/off**: Toggle the corresponding bit in the `OUT` register. If the LED is active low, keep in mind that writing `0` might turn the LED on.
   - **Step C: Delay or wait**: Insert a small delay (for instance, a software loop or a timer-based delay) so you can see the LED turning on/off at a visible pace.

6. **Translating Spec Details into Rust**
   - Rust lets you safely handle hardware registers via crates like `volatile-register`, or you can do raw pointer casts in `unsafe` blocks.
   - From the Product Specification’s addresses, determine the exact base + offset. For example, if GPIO is at `BASE + 0x504` for the `OUT` register, then your pointer for writing might be `(BASE as *mut u32).add(0x504 / 4)`, and so on.
   - Pay careful attention to:
     - Endianness (Cortex-M is little-endian, but you typically don’t have to do anything special for that beyond normal pointer writes).
     - Access sizes (32-bit vs. 8-bit). The register description will tell you the correct size to write.
     - Atomic set/clear registers: Some Nordic chips have separate registers for setting and clearing bits in one operation (e.g., `OUTSET`, `OUTCLR`). If they exist, the Product Specification will mention them. They can be more convenient for toggling bits without risking read-modify-write issues.

7. **Consult Example Code (But Don’t Copy)**
   - Even though you want to do it without a HAL, it can be enlightening to see how Nordic’s official SDK or community examples manipulate the registers. Skim them to confirm you’re looking at the correct registers and bits. Just use them as reference, not a direct code copy.

8. **Iterate & Validate**
   - Test each step. If the LED never lights, re-check:
     - Did you configure the correct pin (maybe the DK LED is actually on another pin)?
     - Are you toggling the correct bit in the `OUT` register?
     - Did you account for active-low or active-high?
   - Use a debugger or RTT prints (like you’re already using) to confirm you’re reaching the toggling logic.

By repeatedly going through this “find docs → locate register → confirm pin → write bits → test” process, you develop the embedded engineer’s muscle memory. Over time, you’ll instinctively know to reach for the product specification and DK hardware manuals first, find your relevant registers, carefully read their bit definitions, and do minimal “unsafe” register writes to achieve your goal.
