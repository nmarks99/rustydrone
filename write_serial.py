#!/usr/bin/env python3
import serial
ser = serial.Serial('/dev/ttyUSB0')
print(ser.name)
ser.write(b'hello\0')
ser.close()
