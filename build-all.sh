cd go-detour
go build -buildmode=c-shared -o libgo-detour.so
cd ../go-target
go build
cd ../hooks
cargo +nightly build


