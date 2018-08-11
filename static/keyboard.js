class Keyboard {
    constructor() {
        this.downKeys = [];
        this.aliases = [];
    }

    trackKey(keyCode, alias) {
        this.downKeys[keyCode] = false;
        window.addEventListener("keydown", this.onKeyDown.bind(this));
        window.addEventListener("keyup", this.onKeyUp.bind(this));
        if (alias !== undefined) {
            this.aliases[alias] = keyCode;
        }
    }

    isKeyDown(key) {
        if (key in this.downKeys) {
            return this.downKeys[key];
        } else if (key in this.aliases) {
            return this.downKeys[this.aliases[key]];
        }
    }

    onKeyDown(keyEvent) {
        if (keyEvent.code in this.downKeys) {
            this.downKeys[keyEvent.code] = true;
        }
    }

    onKeyUp(keyEvent) {
        if (keyEvent.code in this.downKeys) {
            this.downKeys[keyEvent.code] = false;
        }
    }

}