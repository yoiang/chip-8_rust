import { isArrayContentsEqual, renderHexValue } from './Utility'

class DisassemblerLine extends HTMLElement {
    constructor() {
      super();

      this.isCurrentLine = false;
      this.location = 0;
      this.value = [0];
    }
  
    connectedCallback() {
      this.isCurrentLine = this.getAttribute("isCurrentLine") || false;
      this.location = this.getAttribute("location") || 0;
      this.value = this.getAttribute("value") || [0];

      this.render();
    }

    update(new_is_current_line, new_location, new_value) {
      if (typeof new_is_current_line !== 'boolean') {
        console.error("Unexpected disassembler line is current line type: ", typeof new_is_current_line);
        return;
      }

      if (typeof new_location !== 'number') {
        console.error("Unexpected disassembler line location type: ", typeof new_location);
        return;
      }
      
      if (!Array.isArray(new_value)) {
        console.error("Unexpected disassembler line value type: ", typeof new_value);
        return;
      }
      
      if (this.isCurrentLine === new_is_current_line && this.location === new_location && isArrayContentsEqual(this.value, new_value)) {
          return;
      }

      this.isCurrentLine = new_is_current_line;
      this.location = new_location;
      this.value = new_value;

      this.render();
    }
  
    render() {
      this.innerHTML = `
        <div class="disassembler_line${this.isCurrentLine ? " current_disassembler_line" : ""}">
            ${renderHexValue(this.location, 4)}: ${renderHexValue(this.value[0]<< 8 | this.value[1], 4)}
        </div>
        `;
    }
  }
  
  customElements.define("disassembler-line", DisassemblerLine);