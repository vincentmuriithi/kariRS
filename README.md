# kariRS   
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
![Version](https://img.shields.io/badge/version-1.0.0-orange)

kariRS is a Rust-based embedded framework that provides an Arduino-style setup() and loop() programming model for microcontrollers.

It allows developers to write firmware in Rust using familiar Arduino concepts such as **pin configuration, serial communication, and repeated execution while benefiting from Rust’s memory safety, strong typing, and modern tooling**.

kariRS maintains the simplicity of Arduino while enforcing clearer structure and safer execution under the hood.

Internally, kariRS owns the main execution loop and invokes user-defined loop logic, allowing the framework to maintain control over timing, scheduling, and hardware safety.

## 🚧 Project Status
kariRS is under active development.
The public API, execution model, and configuration format are considered **stable**, while internal implementations are still evolving.
Documentation and examples are published early to establish a clear development contract and guide future releases.


## 📚 Table of Contents

- [🚀 Installation](#-installation)
- [⚙️ Project Configuration (kari.toml)](#project-configuration-karitoml)
- [🧩 Supported Boards](#-supported-boards)
- [kariRS CLI](#karirs-cli)
  - [1. `kari new`](#1-kari-new)
  - [2. `kari build`](#2-kari-build)
  - [3. `kari flash`](#3-kari-flash)
  - [4. `kari upload`](#4-kari-upload)
  - [5. `kari monitor`](#5-kari-monitor)
  - [6. `kari update`](#6-kari-update)
  - [7. `kari detect`](#7-kari-detect)
  - [8. `kari doctor`](#8-kari-doctor)
  - [9. `kari add`](#9-kari-add)
  - [10. `kari remove`](#10-kari-remove)
- [Application Structure](#application-structure)
  - [1. Initialization Phase (`#[init]`)](#1-initialization-phase-init)
  - [2. Runtime Loop (`#[run]`)](#2-runtime-loop-run)
  - [3. Execution Model Summary](#3-execution-model-summary)
  - [4. Design Rationale](#4-design-rationale)
  - [5. Rules and Guarantees](#5-rules-and-guarantees)
- [Core Macros](#core-macros)
- [Advanced Macros](#advanced-macros)
- [API Macros (Hardware & Higher-Level Helpers)](#api-macros-hardware--higher-level-helpers)
- [kariRS Hardware Abstraction Layer (HAL)](#karirs-hardware-abstraction-layer-hal)
- [kariRS HAL Function Reference](#karirs-hal-function-reference)
- [🔨 Usage](#-usage)
- [📜 License](#-license)
- [⚖️ Trademark Notice](#-trademark-notice)
- [✨ Author](#-author)


## 🚀 Installation
Follow these steps to get started with kariRS:
### 1. Download kariRS
Download the latest release of kariRS from the GitHub releases page
 as a ZIP file.

### 2. Extract the ZIP
Extract the contents of the ZIP file to a folder of your choice, e.g., C:\kariRS or /home/user/kariRS.

### 3. Add CLI to your PATH
Add the kariRS CLI to your system’s PATH so you can run it from any terminal:
```bash
kariRS/
├── bin/
│   ├── windows/
│   │   └── kari.exe
│   └── linux/
│       └── kari
├── examples/
└── (other internal folders handled automatically)
```

#### <img src="https://www.citypng.com/public/uploads/preview/windows-10-logo-icon-free-png-735811696612207vhxwa5iwgf.png" width=16> Windows

1. Copy the path to the folder containing kari.exe.

2. Open Settings → System → Advanced → Environment Variables.

3. Under User variables, edit the Path variable and add the folder path.

4. Click OK to save.

#### 🐧 macOS / Linux
```bash
export PATH=$PATH:/path/to/kariRS
```
Add the above line to your .bashrc, .zshrc, or .profile to make it permanent.

#### 4. Verify Installation
Open a terminal and run:
```bash
kari doctor
```
This command will check your environment, install any missing dependencies, and ensure kariRS is ready to use.

## Project Configuration (kari.toml)
kariRS projects are configured using a `kari.toml` file.   
This file defines the target board, serial settings, programmer options, and project dependencies.  
The file is automatically generated and configured for you when you create a new project but you can </br>modify the fields  or remove the optional ones as per your project needs.  

It plays a similar role to `Cargo.toml` in Rust or `platformio.ini` in PlatformIO.

### Example `kari.toml`
```toml
[project]
name = "blinky"
board = "uno"
baudrate = 9600
port = "COM12"  # linux example looks like "/dev/ttyUSB0"
usbasp = false

[dependencies]
kari_gpio = "0.1"
kari_time = "0.1"
```

### `[project]` Fields

| Field      | Type    | Required     | Description |
|-----------|---------|--------------|-------------|
| `name`     | string  | ✅ Yes        | Project name |
| `board`    | string  | ✅ Yes        | Target board (`uno`, `mega`, `nano`, `leonardo`, `pro-mini`) |
| `baudrate` | integer | ❌ Optional   | Serial baud rate |
| `port`     | string  | ❌ Optional   | Serial or upload port |
| `usbasp`   | boolean | ❌ Optional   | Enable USBasp programmer |

### `[dependencies]`
Dependencies are optional and allow you to include kariRS-compatible libraries.   
This section is reserved for **future expansion** and forward compatibility with the kariRS ecosystem.   
At present, no official kariRS libraries are distributed through this mechanism.
```toml
[dependencies]
kari_i2c = "0.1"
kari_spi = "0.1"
```


## 🧩 Supported Boards

kariRS currently targets **AVR microcontrollers** and supports the following Arduino-compatible boards:

| Board | MCU | Status |
|------|-----|--------|
| Arduino Uno | ATmega328P | ✅ Stable |
| Arduino Nano | ATmega328P | ✅ Stable |
| Arduino Mega 2560 | ATmega2560 | ✅ Stable |
| Arduino Pro Mini | ATmega328P | ✅ Stable |
| Arduino Leonardo | ATmega32U4 | ✅ Stable |

> ℹ️ Boards sharing the same MCU are supported through a common HAL implementation.


## kariRS CLI
### 1. `kari new`
Creates a new kariRS project.  
Currently supported board names are: uno, mega, nano, leonardo
```bash
kari new <name> [--board <board>]
```
Arguments
- \<name> – Name of the project to create

Options
- -b, --board <board> – Target board (default: uno)

Example
```bash
kari new blink --board mega
```

### 2. `kari build`
Builds the current kariRS project for the configured target board. 
```bash
kari build
```

### 3. `kari flash`
Flashes the project to the target device.
```bash
kari flash [--port <port>] [--baudrate <baud>] [--usbasp]
```

Options
- -p, --port <port> – Serial port (e.g. /dev/ttyUSB0, COM3)    
- -b, --baudrate <baud> – Baud rate (default: 115200 = board default)    
- --usbasp – Flash using USBasp programmer

Example
```bash
kari flash --port COM3
kari flash --usbasp
```

### 4. `kari upload`
Build and flashes the project to the target device.
```bash
kari upload [--port <port>] [--baudrate <baud>] [--usbasp]
```

Options
- -p, --port <port> – Serial port (e.g. /dev/ttyUSB0, COM3)    
- -b, --baudrate <baud> – Baud rate (default: 115200 = board default)    
- --usbasp – flash using USBasp programmer

Example
```bash
kari upload --port COM3
kari upload --usbasp
```

### 5. `kari monitor`

Opens a serial monitor to the connected device.
```bash
kari monitor [--port <port>] [--baudrate <baud>]
```

Options
- -p, --port <port> – Serial port  
- -b, --baudrate <baud> – Baud rate

Example
```bash
kari monitor --port COM3 --baudrate 9600
```

### 6. `kari update`
Updates a field in kari.toml.
```bash
kari update <field> <value>
```

Example
```bash
kari update board mega
kari update baudrate 115200
```

### 7. `kari detect` 
Automatically detects connected boards and serial ports.
```bash
kari detect
```
### 8. `kari doctor`
Used to inspect your environment and install required dependencies to get you started on kariRS.
This is the first command to run after installing kariRS.
Syntax:
```bash
kari doctor
```

### 9. `kari add`
Adds a dependency, library, or module to the project (in kari.toml).
```bash
kari add <name>
```

Example
```bash
kari add ds18b20
kari add servo@1.2.0
```

### 10. `kari remove`
Removes a previously added dependency or module.
```bash
kari remove <name>
```
Example
```bash
kari remove ds18b20
```

## Application Structure
kariRS follows an Arduino-style application structure based on two clearly defined phases:    
Initialization (#[init]) — runs once at boot    
Runtime loop (#[run]) — runs repeatedly

### 1. Initialization Phase (#[init])
The #[init] function is the entry point of a kariRS application.   
It is executed once when the microcontroller starts and is responsible for:    
- configuring pins    
- initializing peripherals (GPIO, Serial, I2C, SPI, EEPROM, timers, etc.)    
- setting up application state
- defining the runtime loop logic

Example:
```rust
#[init]
fn setup() {  // function can be named anything; "setup" is just illustrative
    pinMode!(13, OUTPUT);
    Serial!(9600);

    kprintln!("kariRS initialized");

    #[run]
    fn run() { // Same here the function can be named anything the user wants 
        digitalWrite!(13, HIGH);
        delay(500);
        digitalWrite!(13, LOW);
        delay(500);
    }
}
```

### 2. Runtime Loop (#[run])
The #[run] block defines the code that is executed repeatedly, similar to Arduino’s loop().  
Internally, kariRS:    
- creates an infinite loop
- repeatedly invokes the user-defined #[run] logic    
The user does not write an infinite loop themselves.

Example:
```rust
#[run]
fn run() {
    kprintln!("Running...");
}
```
Conceptually, this is equivalent to Arduino’s:
```cpp
void loop() {
    // user code
}
```
But in kariRS:    
- the loop is controlled by the framework
- the user supplies only the loop body

### 3. Execution Model Summary    
The kariRS execution flow can be summarized as:    
```text
Boot
 └─▶ #[init] runs once
       ├─ hardware setup
       ├─ peripheral initialization
       └─ #[run] is registered
             └─▶ framework-owned loop
                   └─▶ #[run] executes repeatedly
```
This design allows kariRS to:    
- maintain Arduino familiarity
- prevent accidental misuse of infinite loops
- retain control over timing and execution

### 4. Design Rationale
**kariRS** intentionally keeps the Arduino programming model to reduce the learning curve, while improving </br> on Arduino’s limitations:    
- clearer structure    
- safer execution    
- framework-controlled lifecycle    
- room for advanced runtime features    

From the user’s perspective, kariRS feels like Arduino.  
Under the hood, it behaves like a structured embedded framework.

### 5. Rules and Guarantees
- Exactly one <span style="color: #45ffcd">#[init] </span> function is allowed.    
- Exactly one <span style="color: #45ffcd">#[run] </span> block is allowed.    
- All executable application code must be defined inside #[init] or #[run].    
- The main execution loop is provided by kariRS, not the user.


## Core Macros
**kariRS** provides Arduino-style macros for GPIO, analog, PWM, and serial operations. All unsafe and     board-specific setup is handled internally, giving a simple and safe interface for embedded Rust    development.  

#### 1. `pinMode!(pin, mode)`  
Configures a pin as digital input, output, analog input, or PWM output.  

Parameters:  
- <span style="color: #45ffcd">pin</span> : literal integer representing the pin number.  
- <span style="color: #45ffcd">mode </span> : one of INPUT, OUTPUT, _INPUT (analog input), PWM.

Example:
```rust
pinMode!(13, OUTPUT);
pinMode!(0, INPUT); // digital input
pinMode!(0, _INPUT);  // analog input
pinMode!(7, PWM);
```
Notes:
- PWM pins are automatically mapped to the correct timer depending on the board.
- _INPUT sets up the pin as analog input.

#### 2. `digitalWrite!(pin, value)`
Sets a digital output pin to HIGH or LOW.    

Parameters:
- <span style="color: #45ffcd">pin</span> : literal integer representing the pin number.  
- <span style="color: #45ffcd">value</span> : HIGH or LOW.

Example:
```rust
digitalWrite!(13, HIGH);
digitalWrite!(12, LOW);
```

#### 3. `digitalRead!(pin)`
Reads a digital input pin, returning HIGH or LOW.

Parameters:    
- <span style="color: #45ffcd">pin</span> : literal integer representing the pin number.

Example:
```rust
let state = digitalRead!(12);
if state == HIGH {
    kprintln!("Button pressed");
}
```

#### 4. `analogWrite!(pin, value)`
Writes a PWM value to a pin (0–255).

Parameters:  
- <span style="color: #45ffcd">pin</span> : literal integer representing a PWM-capable pin.  
- <span style="color: #45ffcd">value</span> : u16 (0–255).  

Example:
```rust
analogWrite!(7, 128); // 50% duty cycle
```

#### 5. `analogRead!(pin)`
Reads an analog input pin.

Parameters:  
- <span style="color: #45ffcd">pin</span> : literal integer representing an analog pin.

Example:
```rust
let val = analogRead!(0);
kprintln!("Analog value: {}", val);
```

#### 6. `Serial!(baud)`
Initializes serial communication over the default UART.

Parameters:   
- <span style="color: #45ffcd">baud</span> : u32 baud rate. 

Example:
```rust
Serial!(9600);
```

#### 7. `kprintln!(...)`
Prints formatted output over serial. Works like Rust’s println!.    

Example:
```rust
kprintln!("Value: {}", analogRead!(0));
```

#### 8. `gen_pins!(name: pin, ...)`
Assigns board pins to named variables for easier reference.

Example:
```rust
gen_pins!(sda: d20, scl: d21);
let i2c = init_i2c(dp.TWI, sda, scl, None);
```
Creates variables sda and scl bound to pins 20 and 21.    
**Note**: pins generated are **input pull up** types and used in special cases or when user needs low level control of input pins. 

## Advanced Macros
**kariRS** provides a set of advanced macros for scheduling, sequencing, and hardware convenience. These are optional but powerful tools for more complex applications.

#### 1. kariAsync!(|| { ... }, interval_ms, task_name)
Schedules a closure to run repeatedly at a fixed time interval. The framework tracks timing internally,   the user does not need to manage `millis()` manually.

Parameters:  
- Closure: the code to run repeatedly `(|| { ... })`
- <span style="color: #45ffcd">interval_ms</span> : u32 interval in milliseconds
- <span style="color: #45ffcd">task_name</span> : identifier for the scheduled task

Example:
```rust
let mut counter = 0;

kariAsync!(|| {
    counter += 1;
    kprintln!("Counter: {}", counter);
}, 500, task1);
```
This will increment and print counter every 500 ms.


#### 2. `kariPulse!([pin_array], pulse_name)`
Toggles all pins HIGH or LOW on each invocation. Useful for flashing LEDs or toggling outputs.  

Parameters:
- <span style="color: #45ffcd">[pin_array]</span> : array of literal pins    
- <span style="color: #45ffcd">pulse_name</span> : identifier for tracking state    

Example:
```rust
kariPulse!([13, 12, 11], pulse_leds);
```
Each call will flip all pins in the array from HIGH to LOW or vice versa.

#### 3. `kariSequential!([pin_array], sequence_name)`
Cycles through a set of pins sequentially. Only one pin is HIGH at a time; the previous pin is   automatically set LOW.

Parameters:
- <span style="color: #45ffcd">[pin_array]</span> : array of literal pins (e.g., [13, 12, 11])
- <span style="color: #45ffcd">sequence_name</span> : identifier for tracking the sequence

Example:
```rust
kariSequential!([22, 24, 25, 26], led_sequence);
```
On each call, the next pin in the array is set HIGH, and the previous pin is set LOW.

#### 4. `kariSPI!(clk, mosi, miso, cs)`
Initializes an SPI peripheral with the specified pins. Returns an SPI object wrapped in `kariSPI`.  

Parameters:    
clk, mosi, miso, cs: literal pin numbers

Example:
```rust
let spi = kariSPI!(13, 11, 12, 10);
```

#### 5. `kariBegin!([pin_array], mode)`
Bulk-configures multiple pins with the same mode in one call.

Parameters:  
- <span style="color: #45ffcd">[pin_array]</span> : array of literal pins  
- <span style="color: #45ffcd">mode</span> : one of OUTPUT, INPUT, _INPUT, PWM

Example:  
```rust
kariBegin!([22, 24, 25, 26, 27], OUTPUT);
kariBegin!([1, 2], _INPUT);
```

#### 6. `#[eeprom(addr)] struct MyStruct { ... }`
Generates methods to save/load a struct to/from EEPROM.   

Parameters:  
- <span style="color: #45ffcd">addr</span> : optional starting EEPROM address

Example:
```rust
#[eeprom(0x20)]
struct Settings {
    brightness: u8,
    volume: u8,
}

let s = Settings { brightness: 128, volume: 255 };
s.save(&mut eeprom, None);
let loaded = Settings::load(&eeprom, None);
```

## API Macros (Hardware & Higher-Level Helpers)
These macros wrap specific hardware functionality or implement common behaviors. They use core macros internally and return objects or closures that integrate with the kariRS runtime.

#### 1. `kariDrive!(motor1_en, motor1_in_a, motor1_in_b, motor2_en, motor2_in_a, motor2_in_b)`
Controls a 2-motor drive system (e.g., for an RC car or robot).  
Returns a kariDrive object with methods like .drive(speed, reverse), .left(angle, reverse),   
.right(angle, reverse).

Parameters:  
- <span style="color: #45ffcd">motor1_en</span> : PWM pin for motor 1 enable  
- <span style="color: #45ffcd">motor1_in_a</span> : digital pin for motor 1 direction A  
- <span style="color: #45ffcd">motor1_in_b</span> : digital pin for motor 1 direction B  
- <span style="color: #45ffcd">motor2_en</span> : PWM pin for motor 2 enable  
- <span style="color: #45ffcd">motor2_in_a</span> : digital pin for motor 2 direction A  
- <span style="color: #45ffcd">motor2_in_b</span> : digital pin for motor 2 direction B

Example:  
```rust
let mut car = kariDrive!(2, 16, 17, 4, 18, 19);
car.drive(120, false);  // Forward
car.drive(140, true); //  Backward
car.right(45, true);    // Right backward
```


#### 2. `kariInfrared!(pin)`
Creates an infrared sensor object. Returns a `kariInfrared` object.

Parameters:  
- <span style="color: #45ffcd">pin</span> : literal integer pin number

Example:  
```rust
let ir = kariInfrared!(12);
ir.on_measure(||{
    kprintln!("Motion detected\r");
    kprintln!("Callback function\r");
});
```

#### 3. `kariJoyStick!(vrx, vry, sensitivity, Option<threshold>)`
Creates a joystick object for X/Y analog input. Returns closures to read X and Y.

Parameters:
- <span style="color: #45ffcd">vrx</span> : analog pin for X-axis
- <span style="color: #45ffcd">vry</span> : analog pin for Y-axis
- <span style="color: #45ffcd">sensitivity</span> : It’s a minimum delta (change) required before reporting a new value.    
Larger sensitivity = less responsive, more filtering.     
Smaller sensitivity = more responsive, but noisier.  
- <span style="color: #45ffcd">threshold</span> : It’s the baseline reference value (center point of the joystick).    
Defaults to 512 (midpoint of a 10‑bit ADC range 0–1023).      
Used to calculate how far the joystick has moved from center.

Example:
```rust
let joy_stick = kariJoyStick!(1, 2, 500, None);

joy_stick.onX(Some(|value: f32|{
    kprintln!("X value is : {}\r", x_str.format_float(value, 3));
}), false)

.onY(Some(|value: f32|{
    kprintln!("Y value is : {}\r", y_str.format_float(value,3));
}), false);
```

#### 4. `kariPH!(signal, Option<iteration_count>)`
Creates a pH sensor reader. Returns object of `kariPH`. Iteration count determines the number of readings to be averaged when measurement is taken.   

Parameters:   
- <span style="color: #45ffcd">signal</span> : analog pin number   
- <span style="color: #45ffcd">iteration_count</span> : optional integer (default 10)

Example:
```rust
let ph_sensor = kariPH!(1);
ph_sensor
    .measure() // executes the measurement
    .onMeasure(|value:f32|{ // passes the value measured in a user defined closure
    kprintln!("{}\r", x_str.format_float(value, 3));
});
```
Notes:
Reads analog pin multiple times for smoothing

#### 5. `kariPIR!(pin)`
Creates a PIR motion sensor object. Returns a `kariPIR` object.

Parameters:  
pin: literal pin number

Example:
```rust
let pir = kariPIR!(12);
 pir.on_measure(||{  // callback function -> called when motion  is detected 
    kprintln!("Motion detected\r");
    kprintln!("Callback function\r");
    }, None::<fn()>)

    .on_measure(||{},
    Some(||{ // fallback function (optional) -> called when no motion has been detected
    kprintln!("No Motion detected\r");
    kprintln!("Fallback function\r");
}));
```

## kariRS Hardware Abstraction Layer (HAL)
**kariRS** has a Hardware Abstraction Layer (HAL) which provides safe, consistent access to low-level microcontroller hardware. It underpins all high-level APIs and macros, handling operations on GPIO, timers, ADC, PWM, I2C, SPI, UART, and EEPROM. By using HAL, developers can interact with hardware in a platform-independent, Rust-idiomatic way without writing direct register-level code.

## kariRS HAL Function Reference

### Global Functions
| Function | Signature | Description |
|----------|-----------|-------------|
| delay | `delay(milliseconds: u32)` | Blocks execution for the specified number of milliseconds. |
| delayMicroseconds | `delayMicroseconds(microseconds: u32)` | Blocks execution for the specified number of microseconds. |
| millis | `millis() -> u32` | Returns the number of milliseconds since the program started. |
| micros | `micros() -> u32` | Returns the number of microseconds since the program started. |
| map | `map(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32` | Maps a floating-point value from one range to another. |
| init_i2c | `init_i2c(twi, sda, scl, speed: Option<u32>) -> I2c` | Initializes an I2C bus with optional speed. |

---

### `kariLcd`
| Method | Signature | Description |
|--------|-----------|-------------|
| new | `kariLcd::new(i2c_instance, address)` | Creates a new `kariLcd` instance. |
| clear | `clear()` | Clears the LCD display. |
| write | `write(text: &str)` | Writes a string to the LCD. |
| set_cursor | `set_cursor(position: u8)` | Sets the cursor position. |
| shift_display | `shift_display(direction: Direction)` | Shifts the display left or right. |
| set_cursor_visibility | `set_cursor_visibility(visibility: Cursor)` | Shows or hides the cursor. |
| set_cursor_blink | `set_cursor_blink(blink: CursorBlink)` | Enables or disables cursor blinking. |
| write_char | `write_char(data: char)` | Writes a single character. |

---

### `kariPID`
| Method | Signature | Description |
|--------|-----------|-------------|
| new | `kariPID::new(set_point, kp, ki, kd)` | Creates a new PID controller. |
| evaluate | `evaluate(feedBack: f32) -> f32` | Calculates PID output based on feedback. |

---

### `kariJoyStick`
| Method | Signature | Description |
|--------|-----------|-------------|
| new | `kariJoyStick::new(read_x, read_y, sensitivity, threshold)` | Creates a joystick handler. |
| onX | `onX(callback: Option<F>, allow_noise: bool)` | Executes callback when X-axis exceeds sensitivity. |
| onY | `onY(callback: Option<F>, allow_noise: bool)` | Executes callback when Y-axis exceeds sensitivity. |

---

### `kariPH`
| Method | Signature | Description |
|--------|-----------|-------------|
| new | `kariPH::new(read_fn, iteration_count)` | Creates a pH sensor handler. |
| measure | `measure()` | Measures the pH value by averaging multiple readings. |
| onMeasure | `onMeasure(callback)` | Executes callback with the measured pH. |

---

### `kariDrive`
| Method | Signature | Description |
|--------|-----------|-------------|
| new | `kariDrive::new(motor1_en, motor1_in_a, motor1_in_b, motor2_en, motor2_in_a, motor2_in_b)` | Creates a dual motor controller. |
| drive | `drive(speed, direction_status)` | Drives both motors at specified speed and direction. |
| right | `right(speed, direction_status)` | Turns right with speed differentiation. |
| left | `left(speed, direction_status)` | Turns left with speed differentiation. |

---

### `kariPIR`
| Method | Signature | Description |
|--------|-----------|-------------|
| new | `kariPIR::new(signal_read_fn)` | Creates a PIR motion sensor handler. |
| on_measure | `on_measure(callback, fallback)` | Executes callback when motion is detected; optional fallback if no motion. |

---

### `kariInfrared`
| Method | Signature | Description |
|--------|-----------|-------------|
| new | `kariInfrared::new(signal_read_fn)` | Creates an infrared sensor handler. |
| on_measure | `on_measure(callback)` | Executes callback when IR signal is detected. |

---

### `kariEEPROM`
| Method | Signature | Description |
|--------|-----------|-------------|
| new | `kariEEPROM::new(eeprom)` | Initializes EEPROM access. |
| write | `write(offset, buf)` | Writes a buffer to EEPROM. |
| read | `read(offset, buf)` | Reads a buffer from EEPROM. |
| write_byte | `write_byte(address, data)` | Writes a single byte. |
| read_byte | `read_byte(address)` | Reads a single byte. |
| erase | `erase(from, to)` | Erases a range of bytes. |
| update | `update(address, buffer)` | Updates EEPROM bytes only if they differ. |
| capacity | `capacity() -> u16` | Returns EEPROM size. |

---

### `SerialListener`
| Method | Signature | Description |
|--------|-----------|-------------|
| new | `SerialListener::new(rx)` | Creates a serial listener. |
| listen | `listen(handler)` | Reads bytes into a buffer and calls handler on complete lines. |

---

### `kariSPI`
| Method | Signature | Description |
|--------|-----------|-------------|
| new | `kariSPI::new(spi, cs)` | Creates an SPI interface with chip select. |
| transfer | `transfer(read, write)` | Performs full-duplex SPI transfer. |
| write | `write(buffer)` | Writes SPI data. |
| read | `read(buffer)` | Reads SPI data. |


## 🔨 Usage
This section demonstrates common usage patterns in kariRS.  
Each example is a complete application using the #[init] and #[run] execution model.

### 1. Basic Example: Blinking an LED
```rust
#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;


#[init]
fn setup() {
    pinMode!(13, OUTPUT);

    #[run]
    fn run() {
        digitalWrite!(13, HIGH);
        delay(500);
        digitalWrite!(13, LOW);
        delay(500);
    }
}
```

### 2. Basic Example: Serial Output & Analog Input
```rust
#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;


#[init]
fn setup() {
    Serial!(9600);
    pinMode!(0, _INPUT); // analog pin

    #[run]
    fn run() {
        let value = analogRead!(0);
        kprintln!("Analog value: {}", value);
        delay(300);
    }
}
```


### 3. Basic Example: Button Input (Digital Read)
```rust
#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;

#[init]
fn setup() {
    pinMode!(12, INPUT);
    Serial!(9600);

    #[run]
    fn run() {
        if digitalRead!(12) == HIGH {
            kprintln!("Button pressed");
        }
    }
}
```

### 4. Basic Example: PWM Output
```rust
#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;

#[init]
fn setup() {
    pinMode!(7, PWM);
    //In uno PWM pins are: 3, 5, 6, 9, 10 and 11

    #[run]
    fn run() {
        analogWrite!(7, 128); // 50% duty cycle
        delay(1000);
    }
}
```


### 5. Basic Example: `kariDrive` (Motor Drive Control)
```rust
#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){
        let delay_time = 900;
        kariBegin!([22, 16, 18, 19, 17], OUTPUT);
        kariBegin!([2, 4], PWM);
        kariBegin!([1, 2], _INPUT);

        Serial!(9600);
      

        kprintln!("Hello from kariRS kprintln macro!");
        kprintln!("Hello from kariRS println macro!");
        kprintln!("Hello from kariRS!\r");
      let mut car = kariDrive!(2, 16, 17, 4, 18, 19);
      
        kprintln!("Entering loop...\r");
        
 

#[run]
fn run(){
              
    car.drive(120, false); // move in forward direction
    delay(delay_time)
    car.drive(120, false); // move in backward  direction
    delay(delay_time)
    car.right(45, false); // move right in forward direction
    delay(delay_time)
    car.right(45, true);  // move right in backward direction
    delay(delay_time)
    car.left(45, false);  // move left in forward direction
    delay(delay_time)
    car.left(45, true);   // move left in backward  direction
    delay(delay_time)
              
}

}
```



### 6. Basic Example: `kariAsync` (Scheduler)
```rust
#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){

    Serial!(9600);


#[run]
fn run(){


    kariAsync!(||{
            kprintln!("schedule1: {}", millis());
            kprintln!("I come after half a second");
    }, 500, task1);

    kariAsync!(||{
            kprintln!("schedule2: {}", millis());
            kprintln!("I come after second");
            
    }, 1000, task2);

}

}
```



### 7. Basic Example: Storing and loading data from the EEPROM using `kariEEPROM`
```rust
#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){
   
    Serial!(9600);


    let mut eeprom = kariEEPROM::new(_kariEEPROM);


    kprintln!("Hello from kariRS kprintln macro!");
    kprintln!("Hello from kariRS println macro!");
    kprintln!("Hello from kariRS!\r");
    kprintln!("{}\r", millis());
    kprintln!("{}\r", micros());

    let num = 600i32;
    let bytes = num.to_le_bytes();
    size_of::<u16>();


    eeprom.write(0x00, &[56, 200, 12]);
    eeprom.write(0x03, &bytes);
    eeprom.capacity();
    kprintln!("The data at 0x00 is: {}\r", eeprom.read_byte(0x00));
    kprintln!("The data at 0x01 is: {}\r", eeprom.read_byte(0x01));
    kprintln!("The data at 0x02 is: {}\r", eeprom.read_byte(0x02));
    kprintln!("The data at 0x03 is: {}\r", eeprom.read_byte(0x03));
    kprintln!("The eeprom capacity is {}\r", eeprom.capacity());

    let mut buf = [0u8; 2];
    eeprom.read(0x03, &mut buf);
    let ret = u16::from_le_bytes(buf);
    kprintln!("The retrieved data is: {}\r", ret);

    /// saving and loading structured data

     #[eeprom(0x00)]
        struct Config {
            id: u16,
            flag: u8,
        }
       

        let config = Config { id: 987, flag: 0};
        config.save(&mut eeprom, None);

        let config2 = Config { id: 2026, flag: 220};
        config2.save(&mut eeprom, Config::next_addr(1));
    


        let restored = Config::load(&eeprom, None);
        kprintln!("Struct data: id - {}, flag - {}\r", restored.id, restored.flag);
        kprintln!("Struct size: {}\r", size_of::<Config>());

        let restored2 = Config::load(&eeprom, Config::next_addr(1));
        kprintln!("Struct2 data: id - {}, flag - {}\r", restored2.id, restored2.flag);
        kprintln!("Struct2 size: {}\r", size_of::<Config>());
        

#[run]
fn run(){}

}
```

### 8. Basic Example: Building a Multi-LED Blinker with `kariPulse`
```rust
#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){

    kariBegin!([13, 22, 24, 25, 26, 27, 28, 29], OUTPUT);

    Serial!(9600);
      

#[run]
fn run(){
    kariAsync!(||{
        kariPulse!{
                [13, 22, 24, 25, 26, 27, 28, 29],
                    pulse2
        };
            
            
    }, 1000, task4);
       

      
      
}

}
```

### 9. Basic Example: Building LED Chaser with `kariSequential`
```rust
#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){

        kariBegin!([22, 24, 25, 26, 27, 28, 29], OUTPUT);
        

#[run]
fn run(){
    kariAsync!(||{
        kariSequential!{
                [22, 24, 25, 26, 27, 28, 29],
                    sequence1
                };
    }, 1000, task3);
}

}
```


### 10. Basic Example: Reading Serial data and printing out in monitor
```rust
#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){
    Serial!(9600);
    kprintln!("Welcome to kariRS\r");   

#[run]
fn run(){

    kari_serial_listener.listen(|data|{
        kprintln!("{}\r", data);
    });
}

}
```


### 11. Basic Example: Counter Using I2C LCD Display `(kariLCD)`
```rust
#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){

        gen_pins!(sda: d20, scl: d21);
        // gen_pins!(sda: a4, scl: a5);  for uno, nano i.e atmega328 MCUs


        let mut current: u32 = 0;
        let mut previous: u32 = 0;
        let mut current2: u32 = 0;
        let mut previous2: u32 = 0;
        let mut counter = 150;


        let i2c = init_i2c(dp.TWI, sda, scl, None);
        //let mut lcd = lcd_init(i2c, None);
        let mut _delay = arduino_hal::Delay::new();

        let mut lcd = kariLcd::new(i2c, None);

        let mut counter_str = kariString::<16>::new(counter);

        lcd.set_cursor_visibility(Cursor::Invisible)
        .set_cursor_blink(CursorBlink::Off)
        .write("kariRS is live!")
        .set_cursor(40).write("powered by kari");

        delay(2000);
        

        Serial!(9600);

        lcd.set_cursor(64).write("Countdown :");
        


#[run]
fn run(){

    current = millis();
    current2 = current;



    if current - previous >= 500 {
            counter -= 1;
            counter_str.update(counter);
            if counter > 0 {
            lcd.set_cursor(75).write("    ").set_cursor(75)
            .write(counter_str.as_str());
            } else{
            counter = 200;
            }
            previous = current;
    }     
}

}
```

### 12. Basic Example: `kariPID` Controller Usage 
```rust
#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){

    Serial!(9600);

    kprintln!("Hello from kariRS!\r");


    let mut feedback = 50.0;
    let mut step = 0;
    let mut correction: f32 = 0.0;

    let mut correction_str = kariString::<64>::new("");
    let mut feedback_str = kariString::<64>::new("");
    let mut step_str = kariString::<64>::new(step);

    let mut pid = kariPID::new(100.0, 0.5, 0.0001, 80.0); // set point is 100 then kp, ki, kd

        
 

#[run]
fn run(){

    kariAsync!(||{

        correction = pid.evaluate(feedback);
        feedback += correction * 0.5;
        step_str.update(step);
        kprintln!("Step: {} - correction: {} - feedback: {}\r",
            step_str.as_str(), correction_str.format_float(correction, 3),
            feedback_str.format_float(feedback, 3)
        );

        step += 1;
    }, 1000, task3);


      
      
}

}
```

### 13. Basic Example: Detecting Motion Using `kariPIR`
```rust
#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){
    pinMode!(12, INPUT);
    Serial!(9600);
    kprintln!("Hello from kariRS!\r");
    let mut pir = kariPIR!(12);

#[run]
fn run(){

    kariAsync!(||{
        pir.on_measure(||{
        kprintln!("Motion detected\r");
        kprintln!("Callback function\r");
        }, None::<fn()>)

        .on_measure(||{},
        Some(||{
        kprintln!("No Motion detected\r");
        kprintln!("Fallback function\r");
        }));  
    }, 10, task4);

      
      
}

}
```

### 14. Basic Example: Measuring PH using `kariPH` API
```rust
#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){

    kariBegin!([1, 2], _INPUT);
    Serial!(9600);


    kprintln!("Hello from kariRS!\r");
    let mut x_str = kariString::<64>::new("");
    let mut y_str = kariString::<64>::new("");

    let mut ph_sensor = kariPH!(1);

    kprintln!("Entering loop...\r");
        

#[run]
fn run(){
    kariAsync!(||{

        ph_sensor
        .measure()
        .onMeasure(|value:f32|{
        kprintln!("{}\r", x_str.format_float(value, 3));
        });

            
    }, 1000, task4);
}

}
```

### 15. Basic Example: Using a JoyStick for Control using `kariJoyStick` API
```rust
#![no_std]
#![no_main]

mod kari_entry;
use kari_entry::*;



#[init]
fn setup(){

    Serial!(9600);

    kprintln!("Hello from kariRS!\r");

    let mut x_str = kariString::<64>::new("");
    let mut y_str = kariString::<64>::new("");

    let joy_stick = kariJoyStick!(1, 2, 500, None);

    kprintln!("Entering loop...\r");
        
 
#[run]
fn run(){
    kariAsync!(||{
            
        joy_stick.onX(Some(|value: f32|{
                kprintln!("X value is : {}\r", x_str.format_float(value, 3));
        }), false)

        .onY(Some(|value: f32|{
                kprintln!("Y value is : {}\r", y_str.format_float(value,3));
        }), false);
            
    }, 1000, task4);  
}

}
```


## 📜 License
This framework is licensed under the Apache License 2.0.
See the full license here: [Apache-2.0 License.](https://opensource.org/licenses/Apache-2.0)

## ⚖️ Trademark Notice
kari and kariRS are trademarks of Vincent Muriithi Karimi.

Use of the name "kariRS" and related branding is permitted only for:
- referring to the official kariRS project
- attribution in unmodified forks
- non-misleading descriptions

Any fork, derivative, or modified version must not:
- use the name "kariRS"
- imply endorsement by the original project
- reuse official logos or branding


## ✨ Author
**Vincent Muriithi Karimi**  
GitHub: [vincentmuriithi](https://github.com/vincentmuriithi)  
Email: kari.clientdesk@gmail.com 