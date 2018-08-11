class Input {
    constructor() {
        this.activeControls = [];
        this.aliases = [];
        this.mousePosition = [0, 0];

        window.addEventListener("keydown", this.onKeyDown.bind(this));
        window.addEventListener("keyup", this.onKeyUp.bind(this));
        window.addEventListener("mousedown", this.onMouseDown.bind(this));
        window.addEventListener("mouseup", this.onMouseUp.bind(this));
        window.addEventListener("mousemove", this.onMouseMove.bind(this));
    }

    trackKey(keyCode, alias) {
        this.aliases[keyCode] = alias;
        this.activeControls[alias] = false;
    }

    trackMouse(button, alias) {
        this.aliases[button] = alias;
        this.activeControls[alias] = false;
    }

    isControlActive(alias) {
        if (alias in this.activeControls) {
            return this.activeControls[alias];
        } else {
            return false;
        }
    }

    onKeyDown(keyEvent) {
        if (keyEvent.code in this.aliases) {
            this.activeControls[this.aliases[keyEvent.code]] = true;
        }
    }

    onKeyUp(keyEvent) {
        if (keyEvent.code in this.aliases) {
            this.activeControls[this.aliases[keyEvent.code]] = false;
        }
    }

    onMouseDown(mouseEvent) {
        let button = mouseEvent.button.toString();
        if (button.toString() in this.aliases) {
            this.activeControls[this.aliases[button]] = true;
        }
    }

    onMouseUp(mouseEvent) {
        let button = mouseEvent.button.toString();
        if (button in this.aliases) {
            this.activeControls[this.aliases[button]] = false;
        }
    }

    onMouseMove(mouseEvent) {
        this.mousePosition = [mouseEvent.clientX, mouseEvent.clientY];
    }
}