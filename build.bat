cargo build --target wasm32-unknown-unknown
cd target\wasm32-unknown-unknown\debug
wasm-bindgen --target web --no-typescript --out-dir . yew_password_generator.wasm
wasm-gc yew_password_generator.wasm
cd ..\..\..
xcopy .\target\wasm32-unknown-unknown\debug\yew_password_generator_bg.wasm .\www /K /D /H /Y
xcopy .\target\wasm32-unknown-unknown\debug\yew_password_generator.js .\www /K /D /H /Y