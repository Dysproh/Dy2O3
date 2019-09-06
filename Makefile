@RUST_TARGET_PATH = $(shell pwd)
TARGET = armv7a-none-eabi

monolith_bin: monolith_e
		arm-none-eabi-objcopy target/armv7a-none-eabi/release/Monolith -O binary bin/monolith.bin

monolith_e:
		xargo build --target $(TARGET) --release

monolith_bin_debug: monolith_e_debug
		arm-none-eabi-objcopy target/armv7a-none-eabi/debug/Monolith -O binary bin/monolith.bin

monolith_e_debug:
		xargo build --target $(TARGET)

clean:
		xargo clean
		rm -f bin/monolith.bin
