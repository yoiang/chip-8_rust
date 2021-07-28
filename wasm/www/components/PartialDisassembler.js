class PartialDisassembler extends HTMLElement {
    constructor() {
      super();

      this.values = [];
    }
  
    connectedCallback() {
      this.render();
    }

    update(new_program_counter_location, new_values) {
        if (this.values === new_values) {
            return;
        }
        
        if (!Array.isArray(new_values)) {
            console.error("Unexpected variable registers values type: ", typeof new_values);
            return;
        }
        
        let resized = this.values.length !== new_values.length;
        
        this.values = new_values;

        if (resized) {
            this.render();
        }

        // TODO: cache
        this.values.forEach((value, index) => {
            const element = document.getElementById(`disassemblerLine${index}`)
            if (!element) {
                // TODO: report
                return;
            }
            if (!element.update) {
                // TODO: report
                return;
            }
            element.update(new_program_counter_location === value.location, value.location, value.value, value.disassembly)
        })
    }
  
    render() {
        const contents = this.values.map((value, index) => `<disassembler-line id="disassemblerLine${index}"></disassembler-line>`)
        this.innerHTML = `
            <div class="contents">
                ${contents.join("")}
            </div>
            `;
    }
  }
  
  customElements.define("partial-disassembler", PartialDisassembler);