# MicroRDK TMP36 module

This module adds support for the TMP36 analog temperature sensor.
Data Sheet: https://www.analog.com/media/en/technical-documentation/data-sheets/tmp35_36_37.pdf

Note: This module should work for other sensors in this family such as the TMP35 and TMP37, but the user will have to use raw "milliv" return values and convert to Celcius themselves.

## Build
See https://docs.viam.com/operate/get-started/other-hardware/micro-module/

## Configure

The TMP36 sensor requires an analog configuration on the board. 

Example:
```
        "analogs": [
          {
            "name": "temp",
            "pin": 35
          }
        ]
```

Sensor configuration:
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| analog_reader   | <string> | Yes | The name of the analog reader on the board configuration. |
| offset   | <float> | No | Correction factor of values returned (in Celcius) |
| num_readings | <int> | No | Number of raw readings to take per returned median value |

Example:
```
{
    "analog_reader": "temp",
    "num_readings": 15,
    "offset": 0.4
}
```

Note: TMP36 sensors often lack precision and accuracy. Use the `num_readings` configuration to increase precision by using the median of multiple raw readings. Use the `offset` configuration to calibrate and adjust readings to improve accuracy.

## Returned Values
| Key | Type | Description |
|-----|------|-------------|
| temperature_c | float | Temperature in Celsius with offset applied |
| temperature_raw | float | Raw temperature in Celsius before offset |
| milliv | float | Raw analog reading value in millivolts, median value of multiple measurements |
| num_readings | float | Number of readings taken for this measurement |
