# Video connector tool

This project aims at building a tool in Rust to grab videos/images from USB cams as well as stream sources like RTSP. Idea is to have a low level and fast component, that can be remotely controlled to:
* send latest image
* provide an MJPEG stream
* record from a video source
* do some fundamental video transformations like rotation before sending
* a lot of fun stuff, that comes later :)

Please note that everything is developed with and for Linux. 

## Build 

As Rust ships with a excellent depedency and build system, you can build this project by using cargo. This project will most likely use depencies to C/C++ libs like FFMpeg and OpenCV. So prerequisites will be listed here. For the time being you only need Cargo and Rust - and an internet connection :)

More details to follow...

Prequisites on Ubuntu
* apt install build-essentials
* apt install clang


## Run
Rust builds static binaries. Running Cargo will result in a binary in the target subfolder. More details to follow...

## Project structure

* basic config via TOML
* v4l to capture from local video devices
* retina to get RTSP streams
* a command interface (most likely gRPC) to remote control connector

## License
MIT. Means, when you use anything from this code, you do so on your own risk. 