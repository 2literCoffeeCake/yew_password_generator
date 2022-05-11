import { enable_components, default as init } from './yew_password_generator.js';

async function load_wasm() {
    try {
        console.log("Loading wasm...");
        await init('./yew_password_generator_bg.wasm');
        console.log("Wasm was loaded successfully");
    } catch (e) {
        console.error(e);
    }
}

document.addEventListener("DOMContentLoaded", function () {
    load_wasm().then(enable_components);
});