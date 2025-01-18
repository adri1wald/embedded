# GPIO Pin Configuration Reference

## 1. Diagram Interpretation

In a typical hardware‐level block diagram for a microcontroller GPIO (each “pin” section usually looks similar):

1. **Detection (Sense) Block**  
   - Monitors the pin for edges or levels (depending on configuration).  
   - Can latch events and trigger interrupts or wake-ups.

2. **Configuration Bits**  
   - Stored in registers that determine direction (input or output), pull (up/down/none), drive strength, and sense mode.

3. **GPIO Port Block**  
   - Each physical pin has three main registers/signals:
     - **OUT** – Bit value written when the pin is configured as an output.
     - **IN** – Bit representing the pin’s voltage level when read as an input.
     - **CNF** – Configuration bits that define how the pin behaves (direction, pull, drive, sense).

## 2. Key Terminology

### 2.1 Direction
- **Input**: Microcontroller internal driver is disabled; the pin’s voltage is determined by external circuitry or pull resistors. The device “listens.”
- **Output**: Microcontroller drives the pin to a specific logic level (high or low). This mode is usually combined with a drive strength setting (see [Drive Strength](#23-drive-strength)).

### 2.2 Pull (Pull-Up / Pull-Down / No Pull)
- **Pull-Up**: Connects the pin internally to VDD through a resistor. If nothing else drives the pin, it sits at a high logic level.
- **Pull-Down**: Connects the pin internally to GND through a resistor. If nothing else drives the pin, it sits at a low logic level.
- **No Pull**: Disconnects the internal resistor. If no external driver is present, the pin is floating (voltage is undefined). This can be acceptable in some designs but often leads to unpredictable reads if truly undriven.

**Note**: “Switching in” a pull resistor means enabling that internal path to VDD or GND. “Switching out” means disabling that internal path.

### 2.3 Drive Strength
- Determines how strongly (how much current) the microcontroller can source (drive high) or sink (drive low) on a GPIO pin when configured as an output.
- Common labels include “standard” vs. “high” drive or “low power” vs. “high power.”
- Higher drive strength can drive larger loads or enable faster signal edges but often increases power consumption and noise.

### 2.4 Sense Mode (Edge or Level Sensing)
- **Edge**: Rising (low→high), falling (high→low), or both edges trigger an event.
- **Level**: A continuous high or low level triggers an event.
- Hardware can latch these events to generate interrupts or perform wake‐up sequences.

## 3. Practical Points

- If a GPIO pin is **driven as an output**, disabling a pull resistor doesn’t affect the pin’s voltage because the active driver sets the level.
- If a GPIO pin is **configured as input** with no external driver:
  - **Pull-Up** will keep it at a stable high.
  - **Pull-Down** will keep it at a stable low.
  - **No Pull** leaves it floating, which can produce unpredictable reads.

### Switching Between Pull-Up and Pull-Down
- You can dynamically change the pull configuration to alternate the pin’s default level. However, if you simply want a predictable high or low voltage on an output pin, you typically **drive** it directly rather than toggling pull-ups or pull-downs.
- If your goal is to make the pin cycle between high and low **when it’s not otherwise driven externally**, then rapidly switching the internal pull from pull-up to pull-down can force it high and low in software. But this is an unusual design pattern—normally you’d configure it as an **output** and write 1 or 0 to the output register.

## 4. Summary

1. **Direction** determines if the pin is in input or output mode.  
2. **Pull** uses internal resistors to bias an input pin high or low.  
3. **Drive Strength** sets how much current a pin can source or sink when driving an output.  
4. **Sense Mode** controls whether detection logic monitors edges or levels.  
5. **Pull Resistors** can be switched in or out:
   - Switching in: the internal resistor is actively connected to VDD or GND.
   - Switching out: the internal resistor is disconnected; the pin’s voltage is defined by external factors or other internal drivers.
