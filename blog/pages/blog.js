function getContrastiveColorWithMode(imageUrl, callback) {
    const img = new Image();
    img.crossOrigin = "Anonymous";
    img.src = imageUrl;

    img.onload = function () {
        const canvas = document.createElement("canvas");
        const ctx = canvas.getContext("2d");

        // Define dimensions for cropping the middle section of the image
        const cropWidth = img.width * 0.6; // Middle 60% width
        const cropHeight = img.height; // Full height
        const cropX = img.width * 0.2; // Start after 20%
        const cropY = 0;

        // Set canvas size to cropped region
        canvas.width = cropWidth;
        canvas.height = cropHeight;

        // Draw cropped image region onto the canvas
        ctx.drawImage(img, cropX, cropY, cropWidth, cropHeight, 0, 0, cropWidth, cropHeight);

        const imageData = ctx.getImageData(0, 0, cropWidth, cropHeight);
        const data = imageData.data;

        let totalHue = 0, totalLightness = 0;
        let pixelCount = 0;

        // Convert RGB to HSL and calculate total hue and lightness
        const rgbToHsl = (r, g, b) => {
            r /= 255;
            g /= 255;
            b /= 255;

            const max = Math.max(r, g, b);
            const min = Math.min(r, g, b);
            const delta = max - min;

            let h = 0, l = (max + min) / 2;
            let s = 0;

            if (delta !== 0) {
                s = l > 0.5 ? delta / (2 - max - min) : delta / (max + min);
                if (max === r) h = (g - b) / delta + (g < b ? 6 : 0);
                else if (max === g) h = (b - r) / delta + 2;
                else if (max === b) h = (r - g) / delta + 4;
                h *= 60;
            }

            l = Math.min(l + 0.4, 1); // Increase lightness by 20% because of the container background

            return { h: Math.round(h), s: Math.round(s * 100), l: Math.round(l * 100) };
        };

        // Process pixels and calculate HSL averages
        for (let i = 0; i < data.length; i += 4) {
            const r = data[i];
            const g = data[i + 1];
            const b = data[i + 2];

            const { h, l } = rgbToHsl(r, g, b);
            totalHue += h;
            totalLightness += l;
            pixelCount++;
        }

        // Calculate average hue and lightness
        const avgHue = totalHue / pixelCount;
        const avgLightness = totalLightness / pixelCount;

        // Calculate text lightness based on the average lightness
        let textLightness;
        if (avgLightness < 50) {
            textLightness = Math.min(avgLightness + 80, 100); // Ensure max lightness is 100
        } else {
            textLightness = Math.max(avgLightness - 80, 0); // Ensure min lightness is 0
        }

        // Set text color HSL
        const textColorHSL = {
            h: (avgHue < 180)? avgHue + 180 : avgHue - 180, // Invert hue
            s: 50, // Fixed saturation
            l: Math.round(textLightness) // Adjusted lightness
        };

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

        // Convert text color HSL to RGB
        const textColorRgb = hslToRgb(textColorHSL.h, textColorHSL.s, textColorHSL.l);
        const contrastiveColor = `rgb(${textColorRgb.r}, ${textColorRgb.g}, ${textColorRgb.b})`;

        callback(contrastiveColor);
    };

    img.onerror = function () {
        console.error("Failed to load the image.");
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