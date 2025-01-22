function getContrastiveColorWithMode(imageUrl, callback) {
    const img = new Image();
    img.crossOrigin = "Anonymous"; // Allow cross-origin images
    img.src = imageUrl;

    img.onload = function () {
        // Create a canvas to process the image
        const canvas = document.createElement("canvas");
        canvas.width = img.width;
        canvas.height = img.height;

        const ctx = canvas.getContext("2d");
        ctx.drawImage(img, 0, 0, img.width, img.height);

        // Extract image data
        const imageData = ctx.getImageData(0, 0, img.width, img.height);
        const data = imageData.data;

        let totalR = 0, totalG = 0, totalB = 0;

        // Loop through pixels to calculate the total RGB values
        for (let i = 0; i < data.length; i += 4) {
            totalR += data[i];
            totalG += data[i + 1];
            totalB += data[i + 2];
        }

        // Calculate average RGB
        const pixelCount = data.length / 4;
        const avgR = totalR / pixelCount;
        const avgG = totalG / pixelCount;
        const avgB = totalB / pixelCount;

        // Calculate average luminance of the image
        const calculateLuminance = (r, g, b) => {
            const normalize = (val) => (val / 255 <= 0.03928 ? val / 255 / 12.92 : ((val / 255 + 0.055) / 1.055) ** 2.4);
            return 0.2126 * normalize(r) + 0.7152 * normalize(g) + 0.0722 * normalize(b);
        };
        const backgroundLuminance = calculateLuminance(avgR, avgG, avgB);

        // Decide light or dark mode based on luminance
        const isDarkMode = backgroundLuminance > 0.5; // Luminance > 0.5 -> light background

        // Adjust contrastive color in HSL
        const rgbToHsl = (r, g, b) => {
            r /= 255;
            g /= 255;
            b /= 255;

            const max = Math.max(r, g, b);
            const min = Math.min(r, g, b);
            const delta = max - min;

            let h = 0, s = 0, l = (max + min) / 2;

            if (delta !== 0) {
                s = l > 0.5 ? delta / (2 - max - min) : delta / (max + min);

                switch (max) {
                    case r: h = ((g - b) / delta + (g < b ? 6 : 0)) * 60; break;
                    case g: h = ((b - r) / delta + 2) * 60; break;
                    case b: h = ((r - g) / delta + 4) * 60; break;
                }
            }

            return { h: Math.round(h), s: Math.round(s * 100), l: Math.round(l * 100) };
        };

        const hsl = rgbToHsl(avgR, avgG, avgB);

        // Rotate hue for contrasting color
        const contrastHue = (hsl.h + 180) % 360;
        const contrastSaturation = Math.min(hsl.s + 30, 100);
        const contrastLightness = isDarkMode ? Math.min(hsl.l + 50, 85) : Math.max(hsl.l - 50, 15);

        const hslToRgb = (h, s, l) => {
            s /= 100;
            l /= 100;

            const c = (1 - Math.abs(2 * l - 1)) * s;
            const x = c * (1 - Math.abs((h / 60) % 2 - 1));
            const m = l - c / 2;

            let r = 0, g = 0, b = 0;

            if (h >= 0 && h < 60) { r = c; g = x; b = 0; }
            else if (h >= 60 && h < 120) { r = x; g = c; b = 0; }
            else if (h >= 120 && h < 180) { r = 0; g = c; b = x; }
            else if (h >= 180 && h < 240) { r = 0; g = x; b = c; }
            else if (h >= 240 && h < 300) { r = x; g = 0; b = c; }
            else if (h >= 300 && h < 360) { r = c; g = 0; b = x; }

            r = Math.round((r + m) * 255);
            g = Math.round((g + m) * 255);
            b = Math.round((b + m) * 255);

            return { r, g, b };
        };

        const contrastColorRgb = hslToRgb(contrastHue, contrastSaturation, contrastLightness);

        const contrastiveColor = `rgb(${contrastColorRgb.r}, ${contrastColorRgb.g}, ${contrastColorRgb.b})`;
        callback(contrastiveColor, isDarkMode);
    };
}

function applyContrastiveTextColor(imageUrl) {
    getContrastiveColorWithMode(imageUrl, (contrastiveColor, isDarkMode) => {
        const root = document.documentElement;

        // Apply the chosen text color and mode
        root.style.setProperty("--color-text-light", contrastiveColor);
        root.style.setProperty("--color-text-dark", contrastiveColor);
        root.style.setProperty("--theme-mode", isDarkMode ? "light" : "dark");

        if (isDarkMode) {
            document.body.classList.add("light-mode");
            document.body.classList.remove("dark-mode");
        } else {
            document.body.classList.add("dark-mode");
            document.body.classList.remove("light-mode");
        }
    });
}

// Call the function with your image URL
// applyContrastiveTextColor("{}");