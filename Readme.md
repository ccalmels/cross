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
