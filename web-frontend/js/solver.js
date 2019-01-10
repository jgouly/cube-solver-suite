var wasm;
const stack = [];

var wasm_file = location.origin +
                '/target/wasm32-unknown-unknown/release/web_frontend.wasm';
fetch(wasm_file).then(response =>
  response.arrayBuffer()
).then(bytes =>
  WebAssembly.instantiate(bytes, {
    env: {
      stack_push: (val) => stack.push(val),
    }
  })
).then(results => {
  wasm = results.instance;
});

function get_stack_str(results) {
  const [pointer, len] = [stack.pop(), stack.pop()];
  const buffer = new Uint8Array(
    results.exports.memory.buffer,
    pointer,
    len
  );
  const str = String.fromCharCode(...buffer);
  results.exports.dealloc_str(pointer);
  return str;
}

function create_rust_string(str, results) {
  const encoder = new TextEncoder();
  let encoded_str = encoder.encode(str);

  const string_ptr = results.exports.alloc_rust_string(encoded_str.length);
  const string_mem = results.exports.get_rust_string_ptr(string_ptr);

  const asBytes = new Uint8Array(results.exports.memory.buffer, string_mem,
                                 encoded_str.length);
  asBytes.set(encoded_str);
  return string_ptr;
}

self.onmessage = function(msg) {
  let scramble = create_rust_string(msg.data.scramble, wasm);
  let orientations = msg.data.orientations;
  console.time("solve_fb");
  wasm.exports.solve_fb(scramble, orientations);
  console.timeEnd("solve_fb");
  wasm.exports.dealloc_rust_string(scramble);
  self.postMessage(JSON.parse(get_stack_str(wasm)));
}
