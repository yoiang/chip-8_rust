import { renderNumberValue } from './Utility'

class IndexRegister extends HTMLElement {
    constructor() {
      super();

      this.value = 0;
    }
  
    connectedCallback() {
      this.render();
    }

    update(new_value) {
        if (this.value == new_value) {
            return;
        }
        
        if (typeof new_value != 'number') {
            console.error("Unexpected index register value type: ", typeof new_value);
            return;
        }
        
        this.value = new_value;

        this.render();
    }
  
    render() {
      this.innerHTML = `
        <div class="contents">
            ${renderNumberValue("INDEX", this.value, 4)}
        </div>
        `;
    }
  }
  
  customElements.define("index-register", IndexRegister);