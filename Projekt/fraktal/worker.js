// worker.js
self.onmessage = function(event) {
    const { id, x_min, x_max, y_min, y_max, iterations, width, height, startY, endY } = event.data;

    const chunkHeight = endY - startY;
    const pixels = new Uint8ClampedArray(width * chunkHeight * 4);

    let index = 0;

    for (let y = startY; y < endY; y++) {
        const cy = y_max - (y / height) * (y_max - y_min);

        for (let x = 0; x < width; x++) {
            const cx = x_min + (x / width) * (x_max - x_min);

            let zx = 0.0;
            let zy = 0.0;
            let zx2 = 0.0;
            let zy2 = 0.0;
            let iter = 0;

            while (zx2 + zy2 <= 4.0 && iter < iterations) {
                zy = 2.0 * zx * zy + cy;
                zx = zx2 - zy2 + cx;
                zx2 = zx * zx;
                zy2 = zy * zy;
                iter++;
            }

            if (iter === iterations) {
                pixels[index++] = 0;
                pixels[index++] = 0;
                pixels[index++] = 0;
                pixels[index++] = 255;
            } else {
                pixels[index++] = 255;
                pixels[index++] = 255;
                pixels[index++] = 255;
                pixels[index++] = 255;
            }
        }
    }

    // Odsyłamy wynik z powrotem używając Transferable Objects (zero kopiowania!)
    self.postMessage({ pixels, startY }, [pixels.buffer]);
};