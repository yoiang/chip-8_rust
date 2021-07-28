import { renderNumberValue } from './Utility'

class VariableRegister extends HTMLElement {
    constructor() {
      super();

      this.index = 0;
      this.value = 0;
    }
  
    connectedCallback() {
      this.index = this.getAttribute("index");

      this.render();
    }

    update(new_value) {
        if (this.value == new_value) {
            return;
        }
        
        if (typeof new_value != 'number') {
            console.error("Unexpected delay timer value type: ", typeof new_value);
            return;
        }
        
        this.value = new_value;

        this.render();
    }
  
    render() {
      this.innerHTML = `
        <div class="contents">
            ${renderNumberValue(`V${this.index}`, this.value, 2)}
        </div>
        `;
    }
  }
  
  customElements.define("variable-register", VariableRegister);