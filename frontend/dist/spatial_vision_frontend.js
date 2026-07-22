const wasmModule = new WebAssembly.Module(await fetch('./pkg/spatial_vision_frontend_bg.wasm').then(r => r.arrayBuffer()));
console.log('Spatial Vision WASM loaded');
