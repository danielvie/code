## search wifi

```bash
sudo iwlist wlan0 scan
```

## connect to wifi

```bash
sudo vim /etc/wpa_supplicant/wpa_supplicant.conf
```


## add a ssh key
in `~/.ssh`
```bash
echo "your_public_key" >> ~/.ssh/authorized_keys
```

Set Correct Permissions
```bash
chmod 700 ~/.ssh
chmod 600 ~/.ssh/authorized_keys
```

### Possible solution for possible Raspberry Pi Zero 2 W disconnects
sudo iw dev wlan0 get power_save
and getting

power_save: on
in response. The solution to that seems to be to add

/sbin/iw dev wlan0 set power_save off

# install wiringPi

## fetch the source
sudo apt install git
git clone https://github.com/WiringPi/WiringPi.git
cd WiringPi

## build the package
./build debian
mv debian-template/wiringpi-3.0-1.deb .

## install it
sudo apt install ./wiringpi-3.0-1.deb

# use usb / serial with raspberry

in `cmdline.txt`, after `rootwait`, add
```bash
modules-load=dwc2,g_cdc
```

in `config.txt`, add
```bash
dtoverlay=dwc2
```

## install "cdc Composite Gadget"
Go to device manager & select the device. Update driver -> Browse my computer -> Let me pick -> Ports COM/LPT -> Microsoft -> USB Serial Device
