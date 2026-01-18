# SK9822 LED Matrix

## Project Description
This application allows for the control of a 14x14 SK9822 adressable LED matrix. The project includes the following examples:
- All DarK: Turns off all LEDs
- All White: Turns on all LEDs for 1 row
- Demo Mode: Cycles through some preloaded animations
- Full Matrix White: Turns on all LEDs in the matrix
- Modular test: Test of running a singular animation
- SK9822 Server: Allows for changing animations via a server hosted on port 4000

## Required Hardware
- Device running embedded linux with an availible SPI bus
- 14x14 matrix of SK9822 Addressable LEDs
- External power supply for matrix (I used 100W)

## Connections

### Pi
| Pi Physical GPIO Pin | Purpose | Connection |
| :------: | :------: | :------: |
| 6 | GND | External Power Supply GND |
| 19 | GPIO 10 MOSI | LED Matrix LED 0 DI
| 23 | GPIO 11 SCLK | LED Matrix CI

### LED Matrix
| LED Pin | Connection |
| :------: | :------: |
| +5V | +5V from external power supply |
| GND | External Power supply GND |
| LED 0 DI | Pi pin 19 |
| LED 0 CI | Pi pin 23 |

> [!NOTE]
> The LED rows are snaked. As in the the last LED of row 0 (LED 13) is connected to the last LED of row 1. The first
> LED of row 1 is connected to the first LED of row 2, and the pattern continues. This is done to minimize distance
> the signals need to travel

> [!NOTE]
> +5V and GND from the external power supply were connected at row 0, row 5, and row 10. This was done to prevent
> LED dimming in the later rows.

## Build and run
For simplicity, a build.py script was made to automate the build process. It is recommended to perform this on the host.
```bash
python3 build.py
```

Then transfer the generated sk9822-led-module to the target. The directory will contain the executables and all neccessary
components.

### Control the LED
- Go to "localIPAddress:4000/" in any web browser