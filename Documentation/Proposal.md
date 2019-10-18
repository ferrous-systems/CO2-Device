# Project Proposal

This document describes how I made the design decisions. The things to consider are:

- maintainability/simplicity
- energy consumption
- work hours
- material costs


## Sensor Modules

The Ferrous/Asquera Office has four office rooms, each of them will get a sensor module. The first idea was to have different sensor modules in each room. The conference room will get a more sophisticated one with a display, a buzzer and LED indicators, while the other rooms will get simpler versions, with only LED indicators and a buzzer.
A first cost analysis showed, that a module with a display will cost about 200€, a simpler one 145€. Having two simpler modules will save 110€ material costs, while designing a second case will cost more money in engineering hours. The module in the Ferrous Room will be combined with the raspberry pie, and will look different.


### Display Type

For the display I had two choices: A small touchscreen vs. ePaper display and buttons. While a touch screen offers a minimal design and infinite possibilies for interface design, it consumes more energy then an epaper display. A solution would be to combine the touch screen with a motion sensor that indicates if people are present in the room, and only switch the screen on if people are there.
I decided to go for the epaper display, because interface design is restricted to what you can do with no more then three buttons, to keep it simple.


### Interface Design

The interfaces has one button, a display and three RGB LEDs. There are two basic modes: A clock only and automatic switch between displaying Temperature, humidity and CO2 values every 5 minutes. A pushing the button long will change one mode to the other. When in Sensor display mode, pushing the button short, the next value will display.

The RGB LEDs will indicate CO2 levels like a traffic light:

- under 650 ppm -> green
- under 1000 ppm -> Yellow
- above 1000 ppm -> Red

Alternatives: One RGB LED or a cluster of them changing from green over yellow to red.

I opt for RGB LEDs as they can also be used for indicating errors with other colors.

A buzzer will give a short buzz at 1000ppm. The buzzer is then paused for 15 minutes, no matter the CO2 level. If after this period, the level has not gone down, it will go off again.  


### Casing

The casing has to have an opening Large enough to allow fast change of ambient air, but the sensor inside has to be protected from direct air flow and heat sources.

The casing will be made out of antistatic anti reflex acrylic glass, with 4 metal spacers with 8 screws. Further screws are needed to mount the Dev board, the sensor and the display to the backside of the case.

Cable, length according to placement.  


### Wall mounting

An idea was to glue the sensors to the wall with tesa power strips. They hold up to 2kg. They will not damage the wall. It should be near a socket.


### Power

The device will be powered via usb and wall plug.


## Network

The Sensors communicate via Radio or BLE to the data station.
To determine, if the network will be a star or a mesh, experiments will have to be conducted.


### Experiments

Questions:
- How many packets are lost at what distance?
- How do I deal with packet loss?

The embedded training can be used for a first round of tests.


### Protocol

- Contains Info about the sending module
- Contains a time stamp
- Contains sensor data
Options
- ACK/NACK is returned
- Encryption
- Authentication


## Raspberry Pie

## Case

Stacked, so sensor is shielded from raspberry pie heat.


## User Input

SSH
Webinterface


### nrf52832 to Raspberry Pie Communication

#### Protocol

## Data Display

Grafana
What to display

## Data Storage
Backup?
How long is data stored?
