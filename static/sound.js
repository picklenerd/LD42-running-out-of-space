class Sound {
    constructor(file) {
        this.audio = [new Audio(file), new Audio(file), new Audio(file)];
        this.current = 0;
    }

    play() {
        this.audio[this.current].play();
        this.current += 1;
        if (this.current >= this.audio.length) {
            this.current = 0;
        }
    }
}

const hitSound = new Sound("hit.wav");
const shootSound = new Sound("shoot.wav");