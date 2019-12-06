# Project Proposal

This document describes how I made the design decisions. The things to consider are:

- maintainability/simplicity
- energy consumption
- work hours
- material costs

## CO<sub>2</sub> as Indicator for Air Quality

Concentration of Carbon Dioxide is considered to be an indicator for indoor air quality since 1858. Even today, the upper reference value of 1000ppm proposed by Pettenkofer is still valid.

The concentration of Carbon Dioxide from human sources is proportional to other factors which lower the air quality, such as body odor and scents from cosmetics.

The [Umweltbundesamt](https://www.umweltbundesamt.de/sites/default/files/medien/pdfs/kohlendioxid_2008.pdf) proposes the following limits:

* Values below 1000 ppm are considered safe
* Values between 1000 ppm and 2000 ppm are suspicious
* Values above 2000 ppm are unacceptable.


## Sensor Modules

The Ferrous/Asquera Office has four office rooms, each of them will get a sensor module. The first idea was to have different sensor modules in each room. The conference room will get a more sophisticated one with a display, a buzzer and LED indicators, while the other rooms will get simpler versions, with only LED indicators and a buzzer.
A first cost analysis showed, that a module with a display will cost about 200€, a simpler one 145€. Having two simpler modules will save 110€ material costs, while designing a second case will cost more money in engineering hours. The module in the Ferrous Room will be combined with the raspberry pie, and will look different.


### Display Type

For the display I had two choices: A small touchscreen vs. ePaper display and buttons. While a touch screen offers a minimalistic design and infinite possibilies for interface design, it consumes more energy then an epaper display. A solution would be to combine the touch screen with a motion sensor that indicates if people are present in the room, and only switch the screen on if people are there.
I decided to go for the epaper display, because interface design is restricted to what you can do with a limited number of buttons, to keep it simple.


### Interface Design

The interfaces has one button, a display and three RGB LEDs. There are two basic modes: A clock only and automatic switch between displaying Temperature and CO<sub>2</sub> values every 5 minutes. Pushing the button long will change one mode to the other. When in Sensor display mode, pushing the button short, the next value will display.

The RGB LEDs will indicate CO<sub>2</sub> levels like a traffic light:

- under 1000 ppm -> green
- under 2000 ppm -> Yellow
- above 2000 ppm -> Red

Alternatives: One RGB LED or a cluster of them changing from green over yellow to red.

I opt for RGB LEDs as they can also be used for indicating errors with other colors.

Onboard LEDs will indicate measurement ready.

A buzzer will give a short buzz at 2000ppm. The buzzer is then paused for 15 minutes, no matter the CO<sub>2</sub> level. If after this period, the level has not gone down, it will go off again.

A reset button for the device: Onboard Buttons can be accessed through a hole with a paperclip.


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


### Radio Experiments

The setup for the embedded course was used to conduct a first experiment to find out how the radio works inside the office.
The reliability depends highly on how much metal is next to the sending and receiving devices. Best results are optained when both devices are located higher then our monitors. People block the signal transmission, walls and whiteboards do not.

We believe, that with optimized positioning of the devices, there is no need for a mesh topology.


Questions:
- How many packets are lost at what distance?
- How do I deal with packet loss?

- Is packet loss dependent on size of the packet?
- Can packet loss be optimized with other Frequencies?
-

The embedded training radio setup can be used for a first round of tests.


### Protocol[?]

- Contains Info about the sending module
- Contains a time stamp
- Contains sensor data
Options for network health
- ACK is returned
- Encryption
- Authentication


## Raspberry Pie
raspbian

### Case[?]

Stacked, so sensor is shielded from raspberry pie heat.
Case will live on a shelf, that does not exist yet.

### Display[?]

Size of the Display to be determined through trials


### User Input[?]

Buttons on the case to switch between display modes
Reset button


### nrf52832 to Raspberry Pie Communication

SPI, Using Pins 19, 21, 23, 24, 25 on the development board, not the entire connector, as some pins are needed to connect to the sensor.

#### Protocol[x]
raw sensor data is sent, network health is monitored on raspberry pie. All DWM1001 boards carry the same software.

### Network health monitoring[x]

There will be a check if modules are sending, if sent data is received, and if data is authentic.

### Data Display[?]

Data will be displayed with grafana (there is a raspberry pie solution available)

What data should be displayed?
 - network health
 - environmental data
  - CO<sub>2</sub> concentration in each room as trend
  - Temperature
  - Humidity
  - Pressure
  - Average over x period of time
  - deviation from average
 - weather
  - simple weather prediction
  - weather alerts  

How does grafana work?

### Data Storage[?]

How long is data stored? What is the benefit of storing data for longer periods of time?
Daily fluctuation in CO<sub>2</sub> is probably the most interesting, only data that is needed for doing statistics with this over a longer period of time is stored, no masses of raw data. (What data is needed?)

A possiblity is to store data in a cloud. Where?
