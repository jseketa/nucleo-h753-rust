Initial commit.

Repository to document my learning journey:
- embedded rust on the NUCLEO-753ZI board
- unifying my scattered knowledge about microcontroller

Install needed extensions for codium.

codium --install-extension rust-lang.rust-analyzer

codium --install-extension probe-rs.probe-rs-debugger

codium --install-extension tamasfe.even-better-toml

Stop STLinkV3 mass storage mounting:

echo 0483:374e:i | sudo tee /sys/module/usb_storage/parameters/quirks

Without it, it will be automatically remounted every time probe-rs does something.

Found while udiskie was installed.