# extensions

## PlatformIO IDE

similar to arduino

### example config


```yaml
[env:esp32dev]
platform = espressif32
board = esp32dev
framework = arduino
monitor_speed = 115200

lib_deps = qpOasis

build_flags = -Ilib/qpOasis/include
```


## ESP-IDF
        
        