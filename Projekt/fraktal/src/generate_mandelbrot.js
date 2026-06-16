export function generate_mandelbrot_js(x_min, x_max, y_min, y_max, iterations, width, height) {
    // Tworzymy płaską tablicę pikseli (4 wartości na piksel: R, G, B, A)
    const pixels = new Uint8ClampedArray(width * height * 4);

    for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
            // 1. Przeliczenie indeksu piksela na płaszczyznę matematyczną
            const cx = x_min + (x / width) * (x_max - x_min);
            // Pamiętamy o odwróconej osi Y!
            const cy = y_max - (y / height) * (y_max - y_min);

            // 2. Punkt startowy Z = 0 (zoptymalizowany zapis, żeby nie liczyć potęg podwójnie)
            let zx = 0.0;
            let zy = 0.0;
            let zx2 = 0.0;
            let zy2 = 0.0;
            let iter = 0;

            // 3. Pętla czasu ucieczki: Z = Z^2 + C
            while (zx2 + zy2 <= 4.0 && iter < iterations) {
                zy = 2.0 * zx * zy + cy;
                zx = zx2 - zy2 + cx;

                zx2 = zx * zx;
                zy2 = zy * zy;
                iter++;
            }

            // 4. Ustalenie indeksu w płaskiej tablicy
            const pixel_index = (y * width + x) * 4;

            // 5. Kolorowanie
            if (iter === iterations) {
                // Punkt należy do zbioru (wnętrze jest czarne)
                pixels[pixel_index] = 0;       // R
                pixels[pixel_index + 1] = 0;   // G
                pixels[pixel_index + 2] = 0;   // B
                pixels[pixel_index + 3] = 255; // A (pełna widoczność)
            } else {
                // Punkt uciekł – tworzymy prosty niebieskawo-fioletowy gradient z iteracji
                pixels[pixel_index] = 255;      // R
                pixels[pixel_index + 1] = 255;        // G
                pixels[pixel_index + 2] = 255;  // B
                pixels[pixel_index + 3] = 255;               // A
            }
        }
    }

    return pixels;
}