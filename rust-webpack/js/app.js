import { memory } from '../pkg/index_bg';
import { Image } from '../pkg/index'

console.log(memory)

const image = Image.new();
const pixelPointer = image.pixels_ptr();
const pixels = new Uint8Array(memory.buffer, pixelPointer, 6);

console.log(pixels);


function numToHex(v) {
  const hex = v.toString(16);
  console.log(hex)
  return hex.length === 1 ? `0${hex}`: hex;
}

function draw(x, y, color) {
  const ctx = canvas.getContext('2d');
  ctx.fillStyle = `#${numToHex(color[0])}${numToHex(color[1])}${numToHex(color[2])}`;
  ctx.fillRect(x, y, 100, 100);
}

const canvas = document.createElement('canvas');
document.body.appendChild(canvas);
draw(0, 0, pixels.slice(0, 3));
draw(100, 0, pixels.slice(3, 6));