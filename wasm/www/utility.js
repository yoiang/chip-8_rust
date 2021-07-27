
// TODO: flexible key mapping
export const mapKeyEventCodeToKeypadIndex = (key) => {
    switch(key) {
        case "1":
        return 0;

        case "2":
        return 1;

        case "3":
        return 2;

        case "4":
        return 3;


        case "q":
        return 4;

        case "w":
        return 5;

        case "e":
        return 6;

        case "r":
        return 7;


        case "a":
        return 8;

        case "s":
        return 9;

        case "d":
        return 10;

        case "f":
        return 11;


        case "z":
        return 12;

        case "x":
        return 13;

        case "c":
        return 14;

        case "v":
        return 15;
    }
}