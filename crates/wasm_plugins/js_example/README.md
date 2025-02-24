# JS example

run `npm install` to install jco
then

```sh
./node_modules/.bin/jco componentize handle.js --wit ./wit/wasmio.wit -o handle.wasm --disable all
```

the disable all flags will disable all wasi features. HorizonPush has some features enabled.

Other flags
--disable stdio
--disable random
--disable clocks
--disable http


this will generate you a wasm file. Optionally you can strip from debug with

```sh
wasm-tools strip ./handle.wasm -o handlestrip.wasm
```
