export const renderNumberValue = (name, value, hexLength) => {
    return `
        <span class="name">${name}</span>: 
            ${renderHexValue(value, hexLength)} / 
            <span class="decimal_value">${value}</span>
        `
}

export const renderHexValue = (value, hexLength, with0x = true) => {
    return `<span class="hex_value">${with0x ? "0x" : ""}${value.toString(16).padStart(hexLength, "0")}</span>`
}

export const isArrayContentsEqual = (left, right) => {
    if (!Array.isArray(left) || !Array.isArray(right)) {
        // TODO: or throw?
        return false;
    }

    if (left.length !== right.length) {
        return false;
    }

    for(let index = 0; index < left.length; index++) {
        if (left[index] !== right[index]) {
            return false;
        }
    }

    return true;
}