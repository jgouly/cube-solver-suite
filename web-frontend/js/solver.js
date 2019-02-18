delete WebAssembly.instantiateStreaming;

importScripts(location.origin + '/web_frontend.js');

wasm_bindgen(location.origin + '/web_frontend_bg.wasm').then(console.log).catch(console.error);

self.onmessage = function(msg) {
  let orientations = msg.data.orientations;
  let scramble = msg.data.scramble;
  console.time("solve_fb");
  let ret = wasm_bindgen.solve_fb(scramble, orientations);
  console.timeEnd("solve_fb");
  self.postMessage(JSON.parse(ret));
}
