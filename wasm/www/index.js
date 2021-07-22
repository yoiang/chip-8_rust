import { Index } from "../pkg/chip8_wasm";

const pre = document.getElementById("chip8_render-canvas");

let index = null;
let indexReady = false;
let programsList = null;
let programsListReady = false;

const renderLoop = () => {
    pre.textContent = index.render_text();
    index.update();
  
    requestAnimationFrame(renderLoop);
};

const setIndex = (newIndex) => {
    if (indexReady) {
        console.error("setIndex called by index already set: ", newIndex);
        return;
    }

    index = newIndex;
    indexReady = true;

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


/*
        <option value="B3/S23">Life (B3/S23)</option>
        <option value="B2/S">Seeds (B2/S)</option>
        <option value="B1357/S1357">Replicator (B1357/S1357)</option>
        <option value="B0/S0">Radioactive Bacteria (B0/S0)</option>
        <option value="B0/S15">Ancient Techno (B0/S15)</option>
        <option value="B3/S012345678">Life Without Death (B3/S012345678)</option>
        <option value="B3/S1234">Mazectric (B3/S1234)</option>
        <option value="B36/S23">HighLife (B36/S23)</option>
        <option value="B3678/S34678">Day & Night (B3678/S34678)</option>
        <option value="B38/S23">Pedestrian Life (B38/S23)</option>
        <option value="custom">Custom</option>
        */