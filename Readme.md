# Cross compilation for Raspberry pi

Dependencies:
```console
# apt install gcc-arm-linux-gnueabihf libc6-dev-armhf-cross
```

## Using standard Makefile

Classic example using a Makefile:
```console
$ make all
```
For ARM
```console
$ CC=arm-linux-gnueabihf-gcc make all
```

## Using meson

Default
```console
$ meson build && ninja -C build
```

For ARM
```console
$ meson --cross-file cross.ini build && ninja -C build
```
