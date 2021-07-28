import { Index, InterpreterSnapshot } from "../pkg/chip8_wasm";
import { mapKeyEventCodeToKeypadIndex } from "./utility";

const pre = document.getElementById("chip8_render-canvas");

let index = null;
let indexReady = false;
let programsList = null;
let programsListReady = false;
let isPaused = false;

const updateElementById = (id, value) => {
    const element = document.getElementById(id);
    if (!element) {
        console.error("Unable to update element with id '" + id + "', cannot find it");
        return;
    }
    if (typeof element.update !== 'function') {
        console.error("Unable to update element with id '" + id + "', does not contain an update function")
        return;
    }
    element.update(value);
}

const updateSnapshot = () => {
    let snapshot = index.create_interpreter_snapshot(6, 5);

    updateElementById("program_counter", snapshot.program_counter_position);
    updateElementById("index_register", snapshot.index_register_value);
    updateElementById("variable_registers", snapshot.variable_register_values);
    updateElementById("delay_timer", snapshot.delay_timer_value);
    updateElementById("sound_timer", snapshot.sound_timer_value);
    updateElementById("partial_disassembler", snapshot.partial_disassemble);
}

const renderLoop = () => {
    pre.textContent = index.render_text();
    index.update();

    updateSnapshot();
  
    if (!isPaused) {
        requestAnimationFrame(renderLoop); 
    }
};

const togglePause = () => {
    isPaused = !isPaused;
    if (!isPaused && indexReady) {
        requestAnimationFrame(renderLoop);
    }
}

const step = () => {
    if (!isPaused) {
        isPaused = true;
    } else {
        renderLoop();
    }
}

const handleKeydownEvent = (event) => {
    const key = event.key;
    const key_index = mapKeyEventCodeToKeypadIndex(key);
    const result = index.keydown(key_index);
}

const handleKeyupEvent = (event) => {
    const key = event.key;
    const key_index = mapKeyEventCodeToKeypadIndex(key);
    const result = index.keyup(key_index);
}

const setIndex = (newIndex) => {
    if (indexReady) {
        console.error("setIndex called by index already set: ", newIndex);
        return;
    }

    index = newIndex;
    indexReady = true;

    document.addEventListener('keydown', handleKeydownEvent);
    document.addEventListener('keyup', handleKeyupEvent);

    const pauseToggleElement = document.getElementById("play_pause");
    pauseToggleElement.onclick = togglePause;

    const stepElement = document.getElementById("step");
    stepElement.onclick = step;

    requestAnimationFrame(renderLoop);
};

const loadProgram = (fileName) => {
    let result = "";
    return fetch('./' + fileName)
        .then((response) => {
            if (response.status !== 200) {
                throw new Error('Looks like there was a problem. Status Code: ' + response.status);
            }

            let reader = response.body.getReader();
            return reader.read()
                .then(function processText({ done, value }) {
                    if (done) {
                        return;
                    }
                
                    const chunk = value;
                    result += chunk;
                
                    return reader.read().then(processText);
                });
        })
        .then(() => {
            let bytes = result.split(',');
            index.load(bytes);
        })
        .catch((error) => {
            console.error('While loading loading program: ', error);
        });
}

const getProgramsListElement = () => {
    return document.querySelector('#app-programs');
}

const createProgramsListOption = (programsListItem) => {
    let result = document.createElement('option');
    result.value = programsListItem.title;
    result.innerHTML = programsListItem.title;
    return result;
}

const programsListElementSelectedIndex = (index) => {
    // TODO: check out of bounds
    const program = programsList[index]
    loadProgram(`./${program.file}`);
}

const programsListElementChanged = (event) => {
    programsListElementSelectedIndex(event.target.selectedIndex);
    document.activeElement.blur();
}

const setProgramsList = (newProgramsList) => {
    if (!Array.isArray(newProgramsList)) {
        throw new Error(`Expected program list to be an array, received '${typeof newProgramsList}'`);
    }
    if (newProgramsList.length == 0) {
        throw new Error(`Updated program list is empty`);
    }

    let programsListElement = getProgramsListElement();
    while (programsListElement.options.length > 0) {                
        programsListElement.remove(programsListElement.options.length - 1);
    } 

    for (let index = 0; index < newProgramsList.length; index ++) {
        let programsListItem = newProgramsList[index];
        const option = createProgramsListOption(programsListItem);
        programsListElement.appendChild(option);
    }

    programsListElement.onchange = programsListElementChanged;
    programsListElement.selectedIndex = 0;

    programsList = newProgramsList;
    programsListReady = true;

    programsListElementSelectedIndex(programsListElement.selectedIndex);
}

const loadProgramList = () => {
    return fetch('./programs.json')
        .then((response) => {
            if (response.status !== 200) {
                console.log('Looks like there was a problem. Status Code: ' + response.status);
                return;
            }

            return response.json();
        })
        .then((json) => {
            setProgramsList(json);
        })
        .catch((error) => {
            console.error("While loading program list: ", error);
        });
}

loadProgramList();
const newValue = Index.new();
if (newValue && typeof newValue.then == 'function') {
    newValue
        .then(setIndex)
        .catch((error) => {
            console.error("While waiting for Index.new() -> Promise to finish: ", error);
        });
} else {
    setIndex(newValue);
}