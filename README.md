# nicod-cdh
# Getting Started with NanoSat@VT

# Relevant Repositories:
https://github.com/CMUAbstract
https://github.com/CMUAbstract/tartan-artibeus-sw
https://github.com/CMUAbstract/tartan-artibeus-hw

# Make a public SSH key and add it to your github account
https://docs.github.com/en/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent

# Create a git-repos folder in documents
```bash
cd Documents/
mkdir git-repos
cd git-repos/
```

# Git Clone the Repos onto your computer
```bash
sudo apt install git
git clone git@github.com:CMUAbstract/tartan-artibeus-sw.git
git clone git@github.com:CMUAbstract/tartan-artibeus-hw.git
ls -al #List all to check if they are cloned properly
cd tartan-artibeus-sw
git submodule update --init --recursive
cd ta-expt/
cd utilities/
rm -rf stlink
git clone git@github.com:stlink-org/stlink.git

../tartan-artibeus-sw/ # Go up two levels, Use: cd../
sudo apt update 
# Enter your password
sudo apt upgrade
```

# Now download the compiler and usb port channels
```bash
sudo apt install build-essential cmake gcc libusb-1.0-0 libusb-1.0-0-dev libgtk-3-dev
sudo apt autoremove
sudo cp ta-expt/utilities/stlink/config/udev/rules.d/*.rules /etc/udev/rules.d/
cd ta-expt/utilities/stlink/
make clean
make release 
```


# After this completes
```bash
cd ../ # into utilities
wget https://developer.arm.com/-/media/Files/downloads/gnu-rm/9-2020q2/gcc-arm-none-eabi-9-2020-q2-update-x86_64-linux.tar.bz2 # Intel Processor
# OR
wget https://developer.arm.com/-/media/Files/downloads/gnu-rm/9-2020q2/gcc-arm-none-eabi-9-2020-q2-update-aarch64-linux.tar.bz2 # ARM Processor

tar xjf gcc-arm-none-eabi-9-2020-q2-update-x86_64-linux.tar.bz2
# OR
tar xjf gcc-arm-none-eabi-9-2020-q2-update-aarch64-linux.tar.bz2 

cd ../
cd scripts/
source sourcefile.txt
st-flash
cd ../
cd software
cd libopencm3/
make clean
make
```

# After 
```bash
cd ../
cd blink
make
st-flash write blink.bin 0x8000000

# or Blink Fast
cd ..
cd blink-fast
make
st-flash write blink-fast.bin 0x8000000
```

