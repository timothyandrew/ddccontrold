# ddccontrold

[ddccontrol](https://github.com/ddccontrol/ddccontrol) allows controlling external monitor brightness on Linux. It requires `sudo` to use, though, and [various](https://frdmtoplay.com/using-ddccontrol-as-a-non-root-user/) [workarounds](https://github.com/ddccontrol/ddccontrol/issues/5) made no difference on my (Debian) machine. This repo attempts to fix this problem with:

- A daemon that runs as `root`, typically under `systemd`, accepting commands over a domain socket
- A client that does _not_ run as `root` sending commands to this domain socket

The commands in question each use a single byte representing a brightness _or_ contrast setting, along with the value to be set. No other data is passed in, so there's little to no risk of escalation/RCE.

## Installation

```bash
cd path/to/ddccontrold
cargo install --path .
cp dccontrold.service /etc/systemd/system/ddccontrold.service
```

## Usage

```bash
# Server - systemd
sudo systemctl start ddccontrold
sudo systemctl enable ddccontrold

# Server - direct
ddccontrold --mode server

# Client
ddccontrold --brightness <value>
ddccontrold --contrast <value>
```

## Appendix

- [PL](http://plex.local:9999)
