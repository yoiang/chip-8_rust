export const renderNumberValue = (name, value, hexLength) => {
    return `
        <span class="name">${name}</span>: 
            <span class="hex_value">0x${value.toString(16).padStart(hexLength, "0")}</span> / 
            <span class="decimal_value">${value}</span>
        `
}