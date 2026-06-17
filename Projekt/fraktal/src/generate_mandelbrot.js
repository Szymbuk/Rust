// Inicjujemy workery raz, przy ładowaniu pliku
const numWorkers = 16
const workers = [];
for (let i = 0; i < numWorkers; i++) {
    // Ścieżka względem pliku HTML
    workers.push(new Worker('worker.js'));
}

export async function generate_mandelbrot_js(x_min, x_max, y_min, y_max, iterations, width, height) {
    return new Promise((resolve) => {
        const finalPixels = new Uint8ClampedArray(width * height * 4);
        let completedWorkers = 0;

        // Dzielimy ekran na poziome pasy
        const chunkHeight = Math.ceil(height / numWorkers);

        for (let i = 0; i < numWorkers; i++) {
            const startY = i * chunkHeight;
            let endY = startY + chunkHeight;
            if (endY > height) endY = height;

            // Co zrobić, gdy worker skończy swój fragment
            workers[i].onmessage = function(event) {
                const { pixels, startY: returnedStartY } = event.data;

                // Wklejamy fragment do głównej tablicy
                finalPixels.set(pixels, returnedStartY * width * 4);
                completedWorkers++;

                // Jeśli wszyscy skończyli, zwracamy gotowy obraz
                if (completedWorkers === numWorkers) {
                    resolve(finalPixels);
                }
            };

            // Zlecamy pracę
            workers[i].postMessage({
                id: i, x_min, x_max, y_min, y_max, iterations, width, height, startY, endY
            });
        }
    });
}