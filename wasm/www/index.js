import { Index } from "../pkg/chip8_wasm";

const pre = document.getElementById("chip8_render-canvas");

let index = null;
const renderLoop = () => {
    pre.textContent = index.render_text();
    index.update();
  
    requestAnimationFrame(renderLoop);
};

const newValue = Index.new();
if (newValue && typeof newValue.then == 'function') {
    newValue.then((newIndex) => {
        index = newIndex;
        loadProgram().then(() => {
            requestAnimationFrame(renderLoop);
        });
    }).catch((error) => {
        console.error("While waiting for Index.new() -> Promise to finish: ", error);
    });
} else {
    index = newValue;
    loadProgram().then(() => {
        requestAnimationFrame(renderLoop);
    });
}

function loadProgram() {
    return fetch('./Puzzle.ch8')
        .then((response) => {
            if (response.status !== 200) {
                console.log('Looks like there was a problem. Status Code: ' + response.status);
                return;
            }

            let result = "";
            
            let reader = response.body.getReader();
            reader.read().then(function processText({ done, value }) {
                if (done) {
                  return;
                }
            
                const chunk = value;
            
                result += chunk;
            
                return reader.read().then(processText);
              }).finally(() => {
                  let bytes = result.split(',');
                  index.load(bytes);
              });
        })
        .catch((error) => {
            console.log('Fetch Error :-S', error);
        });
}

