class VariableRegisters extends HTMLElement {
    constructor() {
      super();

      this.values = [];
    }
  
    connectedCallback() {
      this.render();
    }

    update(new_values) {
        if (this.values === new_values) {
            return;
        }
        
        if (!(new_values instanceof Uint8Array)) {
            console.error("Unexpected variable registers values type: ", typeof new_values);
            return;
        }
        
        this.values = new_values;

        // TODO: cache
        this.values.forEach((value, index) => {
            const element = document.getElementById(`variableRegister${index}`)
            if (!element) {
                // TODO: report
                return;
            }
            if (!element.update) {
                // TODO: report
                return;
            }
            element.update(value)
        })
    }
  
    render() {
        const contents = [...new Array(16)].map((value, index) => `<variable-register id="variableRegister${index}" index="${index}"></variable-register>`)
        this.innerHTML = `
            <div class="variable-registers">
                ${contents.join("")}
            </div>
            `;
    }
  }
  
  customElements.define("variable-registers", VariableRegisters);