# lirc-client-sys

A rust FFI library for linux's lirc-client library. This is used to interact with infrared emitters and receivers to control IR devices.

Bindings generated, 
```bash
bindgen include/lirc-client.h -o src/bindings.rs
```
