let src = '../target/wasm32-unknown-unknown/release/example.wasm';

window.Module = {}

// Initializing the memory with 20 pages (20 * 64KiB = 1.25 MiB)
const memory = new WebAssembly.Memory({initial: 20});
const imports = {
  env: {
    memory: memory
  }
};

// On instantiation we pass the imports object
fetchAndInstantiate(src, imports)
  .then(mod => {
    Module.memory      = memory;
    Module.alloc       = mod.exports.alloc;
    Module.dealloc     = mod.exports.dealloc;
    Module.dealloc_str = mod.exports.dealloc_str;
    Module.roundtrip   = function(str) {
      let buf = newString(Module, str);
      let outptr = mod.exports.roundtrip(buf);
      let result = copyCStr(Module, outptr);
      Module.dealloc(buf);
      Module.dealloc(outptr);
      return result;
    };

    var output = document.getElementById("tw-content-root");
    output.innerHTML = Module.roundtrip("This string was passed through WebAssembly")
  });