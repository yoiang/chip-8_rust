import { renderNumberValue } from './Utility'

class SoundTimer extends HTMLElement {
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
            console.error("Unexpected sound timer value type: ", typeof new_value);
            return;
        }
        
        this.value = new_value;

        this.render();
    }
  
    render() {
      this.innerHTML = `
        <div class="contents">
            ${renderNumberValue("SOUND", this.value, 2)}
        </div>
        `;
    }
  }
  
  customElements.define("sound-timer", SoundTimer);