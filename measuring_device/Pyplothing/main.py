import datetime as dt
import matplotlib.pyplot as plt
import matplotlib.animation as animation
import serial



# Create figure for plotting
fig = plt.figure()
ax = fig.add_subplot(1, 3, 1)
bx = fig.add_subplot(1, 3, 2)
cx = fig.add_subplot(1, 3, 3)
xs = []
ys_a = []
ys_b = []
ys_c = []

# Initialize communication with TMP102
ser = serial.Serial('/dev/tty.usbmodem0007600081131', 115200, timeout=2) 

# This function is called periodically from FuncAnimation
def animate(i, xs, ys_a, ys_b, ys_c):
    

    line = ser.readline()   # read a '\n' terminated line
    line.decode("utf-8")
    print("{}".format(line))
    # Read temperature (Celsius) from TMP102
    co2_ppm = int(line[0:4])
    temp_c = int(line[5:7])
    humid_percent = int(line[8:10])
    
    print("{} ppm".format(co2_ppm))
    print("{} C".format(temp_c))
    print("{} %".format(humid_percent))

    # Add x and y to lists
    xs.append(dt.datetime.now().strftime('%H:%M:%S.%f'))
    ys_a.append(co2_ppm)
    ys_b.append(temp_c)
    ys_c.append(humid_percent)

    # Limit x and y lists to 20 items
    xs = xs[-20:]
    ys_a = ys_a[-20:]
    ys_b = ys_b[-20:]
    ys_c = ys_c[-20:]

    # Draw x and y lists
    ax.clear()
    bx.clear()
    cx.clear()
    ax.plot(xs, ys_a)
    bx.plot(xs, ys_b)
    cx.plot(xs, ys_c)

    # Format plot
    plt.xticks(rotation=45, ha='right')
    plt.subplots_adjust(bottom=0.30)
    plt.title('CO2 over Time')
    plt.ylabel('CO2 (ppm)')

# Set up plot to call animate() function periodically
ani = animation.FuncAnimation(fig, animate, fargs=(xs, ys_a, ys_b, ys_c), interval=4000)
plt.show()
