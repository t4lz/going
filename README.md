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
