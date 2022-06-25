# raknet-python
[![Build Status](https://img.shields.io/github/workflow/status/b23r0/raknet-python/Rust)](https://github.com/b23r0/raknet-python/actions/workflows/rust.yml)
[![PyPI](https://img.shields.io/pypi/v/raknet-python)](https://pypi.org/project/raknet-python)
[![ChatOnDiscord](https://img.shields.io/badge/chat-on%20discord-blue)](https://discord.gg/ZKtYMvDFN4)

Python bindings to rust-raknet native library.

# Install

```
pip install raknet-python
```

Prebuilds are provided for Python3.8 64-bit Windows/Linux. If a prebuild does not work, please create an issue.

# Build

The RaknetClient and RaknetServer classes are Python wrappers for the internal RaknetSocket and RaknetListener classes implemented in Rust in src/. All methods use asynchronous wrappers.


```py
from raknet_python import RaknetServer,RaknetClient

server = RaknetServer("127.0.0.1:19132")
client1 = RaknetClient("127.0.0.1:19132")

client2 = server.accept()
# first byte must be 0xfe
client1.send([0xfe , 0x01 , 0x01 , 0x01])
buf = client2.recv()
assert buf == [0xfe , 0x01 , 0x01 , 0x01]
```
