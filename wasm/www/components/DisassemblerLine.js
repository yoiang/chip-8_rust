import { isArrayContentsEqual, renderHexValue } from './Utility'

class DisassemblerLine extends HTMLElement {
    constructor() {
      super();

      this.location = 0;
      this.value = [0];
    }
  
    connectedCallback() {
      this.location = this.getAttribute("location") || 0;
      this.value = this.getAttribute("value") || [0];

      this.render();
    }

    update(new_location, new_value) {
      
      if (typeof new_location !== 'number') {
        console.error("Unexpected disassembler line location type: ", typeof new_location);
        return;
      }
      
      if (!Array.isArray(new_value)) {
        console.error("Unexpected disassembler line value type: ", typeof new_value);
        return;
      }
      
      if (this.location === new_location && isArrayContentsEqual(this.value, new_value)) {
          return;
      }

      this.location = new_location;
      this.value = new_value;

      this.render();
    }
  
    render() {
        // TODO: properly render
      this.innerHTML = `
        <div class="contents">
            ${renderHexValue(this.location, 4)}: ${renderHexValue(this.value[0] << 8 + this.value[1], 4)}
        </div>
        `;
    }
  }
  
  customElements.define("disassembler-line", DisassemblerLine);