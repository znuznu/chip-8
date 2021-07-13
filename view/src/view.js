import '../style/stylesheet.css';

const INSTRUCTIONS_PER_STEP = 10;

function mapCodeToKeypadKey(code) {
    return {
        // 1 2 3 C
        Digit1: 0x1,
        Digit2: 0x2,
        Digit3: 0x3,
        Digit4: 0xc,
        // 4 5 6 D
        KeyQ: 0x4,
        KeyW: 0x5,
        KeyE: 0x6,
        KeyR: 0xd,
        // 7 8 9 E
        KeyA: 0x7,
        KeyS: 0x8,
        KeyD: 0x9,
        KeyF: 0xe,
        // A 0 B F
        KeyZ: 0xa,
        KeyX: 0x0,
        KeyC: 0xb,
        KeyV: 0xf
    }[code];
}

async function start() {
    // Unsupported MIME type error
    // const interpreter = await WebAssembly.instantiateStreaming(fetch('src/chip_8.wasm'));
    // const instanceExports = interpreter.instance.exports;

    const response = await fetch("src/chip_8.wasm");
    const buffer = await response.arrayBuffer();
    const interpreter = await WebAssembly.instantiate(buffer);
    const instanceExports = interpreter.instance.exports;

    let requestAnimationFrameID = null;

    const interpreterMemory = new Uint8Array(
        instanceExports.memory.buffer,
        instanceExports.get_memory(),
        4096
    );

    const pixelsMemory = new Uint8Array(
        instanceExports.memory.buffer,
        instanceExports.get_pixels(),
        2048
    );

    const width = instanceExports.get_width();
    const height = instanceExports.get_height();

    const canvas = document.getElementById('chip-8-canvas');
    const ctx = canvas.getContext('2d');
    ctx.fillStyle = 'black';
    ctx.fillRect(0, 0, width, height);

    const loadButton = document.getElementById('btn-load-game');
    loadButton.addEventListener('click', async () => {
        await loadGame(`${document.getElementById('slct-game').value}.ch8`);

        if (requestAnimationFrameID) {
            cancelAnimationFrame(requestAnimationFrameID);
        }

        loop();
    });

    document.addEventListener('keydown', (event) => {
        const keypadKey = mapCodeToKeypadKey(event.code);
        instanceExports.set_key_down(keypadKey);
    });

    document.addEventListener('keyup', (event) => {
        const keypadKey = mapCodeToKeypadKey(event.code);
        instanceExports.set_key_up(keypadKey);
    });

    [
        '1',
        '2',
        '3',
        '4',
        'Q',
        'W',
        'E',
        'R',
        'A',
        'S',
        'D',
        'F',
        'Z',
        'X',
        'C',
        'V'
    ].forEach((key) => {
        const div = document.createElement('p');
        div.className = 'content__kb-layout__keys-grid__item';
        div.innerHTML = `${key}`;
        document.getElementById('kb-layout-grid').appendChild(div);
    });

    [
        'Breakout',
        // 'Danm8ku', need rnd
        'IBMLogo',
        'KeypadTest',
        'Maze',
        'RPS',
        'Snake',
        'SpaceInvaders',
        'Trip8'
    ].forEach((game, index) => {
        const option = document.createElement('option');
        option.innerText = `${game}`;
        option.value = `${game}`;

        if (!index) {
            option.selected = 'selected';
        }

        document.getElementById('slct-game').appendChild(option);
    });

    function loop() {
        for (let i = 0; i < INSTRUCTIONS_PER_STEP; i++) {
            instanceExports.cycle();
        }

        instanceExports.tick();

        render();

        requestAnimationFrameID = window.requestAnimationFrame(loop);
    }

    function render() {
        const imageData = ctx.createImageData(width, height);

        for (let i = 0; i < pixelsMemory.length; i++) {
            // See: https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Pixel_manipulation_with_canvas
            imageData.data[i * 4] = pixelsMemory[i] === 1 ? 255 : 0;
            imageData.data[i * 4 + 1] = pixelsMemory[i] === 1 ? 255 : 0;
            imageData.data[i * 4 + 2] = pixelsMemory[i] === 1 ? 255 : 0;
            imageData.data[i * 4 + 3] = 255;
        }

        ctx.putImageData(imageData, 0, 0);
    }

    async function loadGame(filename) {
        console.log(`Loading game file ${filename}`);

        await fetch(`games/${filename}`)
            .then((f) => f.arrayBuffer())
            .then((buffer) => {
                const game = new DataView(buffer, 0, buffer.byteLength);
                instanceExports.init();

                for (let byte = 0; byte < game.byteLength; byte++) {
                    interpreterMemory[0x200 + byte] = game.getUint8(byte);
                }
            });
    }
}

start().catch((e) => console.log(e));
