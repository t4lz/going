# going
Hook go from rust the easy way.

### Building all componenets:
```
./build-all.sh
```

### Building single components:

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

### Running Test:
```
cd hooks
LD_PRELOAD="target/debug/libhooks.so" ../go-target/go-target
```

### Docker Image
#### Building:
```
docker build -t goli .
```

#### Running docker container:
```
docker run -dit --cap-add=SYS_PTRACE --security-opt seccomp=unconfined --name=goli1 -v=/Users/t4lz/Documents/projects/going:/root/going goli
```
The `--cap-add=SYS_PTRACE --security-opt seccomp=unconfined` part is for lldb.

### Debugging
```
lldb ../go-target/go-target
settings set target.env-vars LD_PRELOAD="target/debug/libhooks.so"
process launch --stop-at-entry
```


### Current status:
Works on ubuntu/arm64, crashes on x86_64.

