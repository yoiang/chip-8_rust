import { renderNumberValue } from './Utility'

class ProgramCounter extends HTMLElement {
    constructor() {
      super();

      this.position = 0;
    }
  
    connectedCallback() {
      this.render();
    }

    update(new_position) {
        if (this.position == new_position) {
            return;
        }
        
        if (typeof new_position != 'number') {
            console.error("Unexpected position counter position type: ", typeof new_value);
            return;
        }

        this.position = new_position;

        this.render();
    }
  
    render() {
      this.innerHTML = `
        <div class="contents">
          ${renderNumberValue("PROGRAM", this.position, 4)}
        </div>
        `;
    }
  }
  
  customElements.define("program-counter", ProgramCounter);