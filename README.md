# DysPROSium Oxide
### Rust for VexV5
###### (aka notnotnotvcs)

## WARNING
This template is really not a good way to program your Robot. Several PROS features do not work at the moment, such as threads, and the mangled mess that is the Bindgen C API bindings is really terrible to use. I mainly made this for educational purposes. If you do choose to use this for competition, I am not responsible for anything, including losing autonomous, the head ref getting mad about you using an unsopported language, or thermonuclear war.

## Installation
This has only been tested on Ubuntu 18.04.3 LTS

Since this is not meant for people who can't figure out how to do things like this on their own, the installation instructions are not super in-depth.

1. Install PROS. You will need a full installation, including the CLI and the atom bootstrapper, and gcc-arm-embedded

2. Install Rustup and Cargo

3. Install the latest nightly Rust toolchain for your development platform using rustup.

4. Using Cargo, install Xargo. This is a modified Cargo that can build for armv7a-none-eabi using the included json file, which is what we use for Vex V5.

5. Clone this repository into the project directory

6. Somehow obtain a project.pros file. This doesn't come with one as I have no idea how to generate one or if it is even needed, other than for uploading to the robot.

7. Get the [cty](https://crates.io/crates/cty) source code and put it in libraries/cty

8. Open an issue with the error you get because I definitely forgot something

## Building
1. Open a command line in your project's root directory

2. Run the command `make`

3. Using the PROS toolbar, because I can't figure out the CLI, upload to the robot

	* If it gives you an error, run the command `sudo chmod a+rw /dev/ttyACM#`, where `#` is the number next to your brain's communications port when you run `prosv5 lsusb`
4. Hope it works

5. Submit an issue because I forgot something

## Dependencies
Dy2O3 uses things from [PROS](https://github.com/purduesigbots/pros), [FreeRTOS](https://freertos.org), [OkapiLib](https://github.com/okapilib/okapilib), and [CTY](https://crates.io/crates/cty). I did  my best to attribute things properly to their authors, but if you have any concerns about this, please open an issue so we can sort this out without any legal action getting involved.
