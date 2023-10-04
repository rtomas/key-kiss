let x_spaces = 64;
let y_spaces = 16;
let block_width, block_height;
let data = [];
let initX = 0,
    initY = 0;
function setup() {
    createCanvas(19, 648);

    block_width = floor((width - initX * 2) / x_spaces);
    block_height = floor((height - initY * 2) / y_spaces);

    for (let pX = 0; pX < x_spaces; pX++) {
        data[pX] = [];
        for (let pY = 0; pY < y_spaces; pY++) {
            data[pX][pY] = true;
        }
    }
}

function draw() {
    Dibujar();
}

function Dibujar() {
    background(220);
    stroke(255);
    print(block_width, block_height);
    for (let pX = 0; pX < x_spaces; pX++) {
        for (let pY = 0; pY < y_spaces; pY++) {
            let black_color = pX % 2 == 0 ? 0 : 35;
            data[pX][pY] ? fill(black_color) : fill(255);

            let px_show = pX * block_width + initX;
            let py_show = pY * block_height + initY;
            rect(px_show, py_show, block_width, block_height);
        }
    }
}

function mouseClicked() {
    let x, y;
    if (mouseX > initX && mouseY > initY) {
        x = floor((mouseX - initX) / block_width);
        y = floor((mouseY - initY) / block_height);
        data[x][y] = !data[x][y];
    }
}

function keyPressed() {
    // Download secret kiss slots paper
    if (key == "s") {
        saveCanvas();
    }
}
