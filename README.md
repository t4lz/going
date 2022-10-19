# going
Hook go from rust the easy way.

Build the go detour lib:
```
cd go-detour
go build -buildmode=c-shared -o libgo-detour.so
```

Build the go target binary:
```
cd go-target
go build
```

Build the hooking rust lib:
```
cd hooks
cargo +nightly build
```

The rust binding for the go-detour lib were created with:
```
bindgen libgo-detour.h -o go_detour_ffi.rs
```

Run:
```
LD_PRELOAD="hooks/target/debug/libhooks.so" go-target/go-target
```

Build docker image for testing on apple chips:
```
docker build -t goli .
```

Run docker container:
```
docker run -dit --name=goli1 -v=/Users/tal/Documents/projects/going:/root/going goli
```


Current status:
Sometimes it doesn't crash.
