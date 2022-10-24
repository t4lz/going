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

Run:
```
cd hooks
LD_PRELOAD="target/debug/libhooks.so" ../go-target/go-target
```

Build docker image for testing on apple chips:
```
docker build -t goli .
```

Run docker container:
```
docker run -dit --cap-add=SYS_PTRACE --security-opt seccomp=unconfined --name=goli1 -v=/Users/t4lz/Documents/projects/going:/root/going goli
```
The `--cap-add=SYS_PTRACE --security-opt seccomp=unconfined` part is for lldb.

debugging:
```
lldb ../go-target/go-target
settings set target.env-vars LD_PRELOAD="target/debug/libhooks.so"
process launch --stop-at-entry
```


Current status:
Reduced POC working, also with many arguments and a return value.

