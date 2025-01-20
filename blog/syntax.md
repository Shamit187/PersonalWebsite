### Markdown-to-HTML Conversion Rules

#### **1. Headers** BlockEnum

- **Syntax**:
  - `# Header 1` → `<div class="header-1">Section</div>`
  - `## Header 2` → `<div class="header-2">Sub Section</div>`
  - `### Header 3` → `<div class="header-3">Sub Sub Section</div>`
  - `#### Header 4` → `<div class="header-4">Bold Title for a paragraph</div>`

---

#### **2. Images** BlockEnum

- **Syntax**:
  ```
  ![alt text](url "100px") {ref: image-id} {cap: This is the image caption.}
  ```
  - Translates to:
    ```html
    <figure class="image" id="image-id">
      <img src="url" alt="alt text" style="width: 100px; aspect-ratio: auto;" />
      <figcaption>This is the image caption.</figcaption>
    </figure>
    ```

---

#### **3. Lists** BlockEnum

- **Syntax**:
  ```
  :::list
  - Item 1
    - Subitem 1.1
    - Subitem 1.2
  - Item 2
  :::
  ```
  - Translates to:
    ```html
    <div class="list">
      <div class="list-item">
        <span class="bullet">•</span> Item 1
        <div class="list-item">
          <span class="bullet">•</span> Subitem 1.1
        </div>
        <div class="list-item">
          <span class="bullet">•</span> Subitem 1.2
        </div>
      </div>
      <div class="list-item">
        <span class="bullet">•</span> Item 2
      </div>
    </div>
    ```

---

#### **4. Tables** BlockEnum

- **Syntax**:
  ```
  :::table {ref: table-id}
  Caption: This is the table caption.
  | Header 1   | Header 2   |
  |------------|------------|
  | Cell 1     | Cell 2     |
  | colspan=2: Combined Cell |
  | rowspan=2: Vertical Cell | Cell 4 |
  :::
  ```
  - Translates to:
    ```html
    <figure class="table" id="table-id">
      <div class="table-caption">This is the table caption.</div>
      <div class="table">
        <div class="table-row">
          <div class="table-cell header">Header 1</div>
          <div class="table-cell header">Header 2</div>
        </div>
        <div class="table-row">
          <div class="table-cell">Cell 1</div>
          <div class="table-cell">Cell 2</div>
        </div>
        <div class="table-row">
          <div class="table-cell" colspan="2">Combined Cell</div>
        </div>
        <div class="table-row">
          <div class="table-cell" rowspan="2">Vertical Cell</div>
          <div class="table-cell">Cell 4</div>
        </div>
      </div>
    </figure>
    ```

---

#### **5. Highlighted Text** 

- **Syntax**: `==highlight=={color:red}` → `<span class="highlight" style="background-color: red;">highlight</span>`
- specific colors:

**1. Pastel Highlights (Soft and Subtle):**
   - LightLavender (#E6E6FA)
   - MintGreen (#98FF98)
   - BabyBlue (#ADD8E6)
   - SoftPeach (#FFDAB9)
   - PaleYellow (#FFFFE0)

**2. Vibrant Highlights (Energetic and Eye-catching):**
   - NeonGreen (#39FF14)
   - BrightYellow (#FFFF00)
   - ElectricPink (#FF1493)
   - FluorescentOrange (#FF4500)
   - TurquoiseBlue (#40E0D0)

**3. Elegant and Muted (Sophisticated):**
   - DustyRose (#DCAE96)
   - SageGreen (#B2AC88)
   - SlateBlue (#6A5ACD)
   - ChampagneGold (#FAD6A5)
   - SoftGray (#D3D3D3)

**4. Modern Minimal (Neutral and Chic):**
   - LightGray (#F5F5F5)
   - Beige (#F5F5DC)
   - SoftWhite (#FAFAFA) with a thin outline
   - BlushPink (#FFC0CB)
   - IceBlue (#E0F7FA)

**5. Contrast Colors for Dark Backgrounds:**
   - LemonYellow (#FFF700)
   - Cyan (#00FFFF)
   - Magenta (#FF00FF)
   - BrightOrange (#FFA500)
   - LightLime (#CCFF99)

---

#### **6. Colored Text**

- **Syntax**:
  - `{color:red}This is red text` → `<span class="text-red">This is red text</span>`
  - `{color:#ff0000}This is custom colored text` → `<span class="text-custom" style="color: #ff0000;">This is custom colored text</span>`

---

#### **7. Explainable Text**

- **Syntax**: `?explainable{text to explain}(hover explanation)` → `<span class="explainable" data-tooltip="hover explanation">text to explain</span>`

---

#### **8. Callouts (Notes/Warnings/Info)** BlockEnum

- **Syntax**:
  ```
  !!! Note
  This is an important note.
  ```
  - Translates to:
    ```html
    <div class="callout note">
      <span class="callout-title">Note</span>
      <div class="callout-body">This is an important note.</div>
    </div>
    ```
  - Variants: `!!! Warning`, `!!! Info`, `!!! Success`.

---

#### **9. Footnotes** BlockEnum

- **Syntax**:
  ```
  Here is a sentence with a footnote.[^1]
  [^1]: This is the footnote explanation.
  ```
  - Translates to:
    ```html
    <div class="paragraph">
      Here is a sentence with a footnote.
      <sup class="footnote" id="fnref-1"><a href="#fn-1">[1]</a></sup>
    </div>
    <div class="footnotes">
      <div class="footnote" id="fn-1">
        <span class="footnote-number">1.</span> This is the footnote explanation.
      </div>
    </div>
    ```

---

#### **10. Collapsible Sections** BlockEnum

- **Syntax**:
  ```
  :::collapse Open for more info
  This content is initially hidden.
  :::
  ```
  - Translates to:
    ```html
    <div class="collapse">
      <button class="collapse-toggle">Open for more info</button>
      <div class="collapse-content">This content is initially hidden.</div>
    </div>
    ```

---

#### **11. Task Lists** BlockEnum

- **Syntax**:
  ```
  - [ ] Incomplete Task
  - [x] Completed Task
  ```
  - Translates to:
    ```html
    <div class="task-item">
      <input type="checkbox" disabled /> Incomplete Task
    </div>
    <div class="task-item">
      <input type="checkbox" disabled checked /> Completed Task
    </div>
    ```

---

#### **12. Inline Math**

- **Syntax**: `$E=mc^2$` → `<span class="math">E=mc<sup>2</sup></span>`

---

#### **13. Block Math** BlockEnum

- **Syntax**:
  ```
  $$ {ref: equation-id}
  \int_a^b x^2 dx
  $$
  ```
  - Translates to:
    ```html
    <div class="math-block" id="equation-id">
      \int_a^b x^2 dx
    </div>
    ```

---

#### **14. References**

- **Syntax**:
  ```
  [ref: image-id]
  [ref: table-id]
  [ref: equation-id]
  ```
  - Translates to:
    ```html
    <a class="reference" href="#image-id">[Image]</a>
    <a class="reference" href="#table-id">[Table]</a>
    <a class="reference" href="#equation-id">[Equation]</a>
    ```

---

#### **15. Text Formatting**

- **Syntax**:
  - **Bold**: `**bold**` → `<span class="bold">bold</span>`
  - **Italic**: `*italic*` → `<span class="italic">italic</span>`
  - **Underline**: `_underline_` → `<span class="underline">underline</span>`
  - **Strikethrough**: `~~strikethrough~~` → `<span class="strikethrough">strikethrough</span>`
  - **Monospace**: `` `monospace` `` → `<span class="monospace">monospace</span>`
  - **Superscript**: `^^superscript^^` → `<span class="superscript">superscript</span>`
  - **Subscript**: `,,subscript,,` → `<span class="subscript">subscript</span>`

---

#### **16. Scripted JS Canvas** BlockEnum

- **Syntax**:
  ```
  :::canvas {id: canvas-id}
  Script:
  function draw(ctx) {
    ctx.fillStyle = "red";
    ctx.fillRect(10, 10, 100, 100);
  }
  :::
  ```
  - Translates to:
    ```html
    <div class="canvas-container">
      <canvas id="canvas-id"></canvas>
      <script>
        (function() {
          const canvas = document.getElementById("canvas-id");
          const ctx = canvas.getContext("2d");
          function draw(ctx) {
            ctx.fillStyle = "red";
            ctx.fillRect(10, 10, 100, 100);
          }
          draw(ctx);
        })();
      </script>
    </div>
    ```

--- 
 
#### **17. Horizontal Line** BlockEnum

-  **Syntax** : `---`  → `<hr class="horizontal-line" />` 

---



#### **18. Audio Stamp**  BlockEnum

- **Syntax :** `@04:45` → `<div class="audio-stamp" id="4-45-audio">`


---

#### **19. Multiple-Choice Questions** BlockEnum
 
- **Syntax**:
  ```
  :::question {ref: question-id}
  Question: What is the capital of France?
  - Option 1: Madrid
  - Option 2: Paris (correct)
  - Option 3: Berlin
  :::
  ```
  - Translates to:
    ```html
    <div class="question" id="question-id">
      <div class="question-text">What is the capital of France?</div>
      <div class="question-options">
        <label>
          <input type="radio" name="question-id" value="1" /> Madrid
        </label>
        <label>
          <input type="radio" name="question-id" value="2" /> Paris
        </label>
        <label>
          <input type="radio" name="question-id" value="3" /> Berlin
        </label>
      </div>
      <button class="check-answer" data-correct="2">Check Answer</button>
      <div class="answer-feedback" style="display: none;"></div>
    </div>
    ```
  - Notes:
    - The `data-correct` attribute on the button indicates the correct option.
    - JavaScript functionality:
      - On clicking **Check Answer**, the script verifies the selected option.
      - Displays feedback like "Correct!" or "Wrong! The correct answer is Paris."


1. **User Selection**:
   - Users select one option using radio buttons.

2. **Answer Checking**:
   - Upon clicking **Check Answer**, the script:
     - Compares the selected option's value to the `data-correct` value.
     - Displays a feedback message:
       - **Correct!** if the answer matches.
       - **Wrong!** with the correct answer if it doesn’t match.

3. **Example Feedback**:
   - If the correct answer is Paris (Option 2), feedback can look like:
     ```html
     <div class="answer-feedback">Correct!</div>
     ```
     or
     ```html
     <div class="answer-feedback">Wrong! The correct answer is Paris.</div>
     ```

---

#### **21. Image Carousel**

- **Syntax**:
  ```
  :::carousel {ref: carousel-id}
  ![Image 1](url1)
  ![Image 2](url2)
  ![Image 3](url3)
  :::
  ```
  - Translates to:
    ```html
    <div class="image-carousel" id="carousel-id">
      <div class="carousel-slide"><img src="url1" alt="Image 1"></div>
      <div class="carousel-slide"><img src="url2" alt="Image 2"></div>
      <div class="carousel-slide"><img src="url3" alt="Image 3"></div>
      <button class="carousel-control prev">‹</button>
      <button class="carousel-control next">›</button>
    </div>
    ```
  - Notes:
    - Requires JavaScript for sliding behavior.
    - Can be extended with captions and autoplay.

---

#### **22. Embedded Videos** BlockEnum

- **Syntax**:
  ```
  :::video {ref: video-id}
  Source: https://example.com/video.mp4
  Caption: This is an embedded video.
  :::
  ```
  - Translates to:
    ```html
    <div class="video" id="video-id">
      <video controls>
        <source src="https://example.com/video.mp4" type="video/mp4">
        Your browser does not support the video tag.
      </video>
      <div class="video-caption">This is an embedded video.</div>
    </div>
    ```

---

#### **23. Quotes with Attribution** BlockEnum

- **Syntax**:
  ```
  :::quote {ref: quote-id}
  "To be or not to be, that is the question."
  - William Shakespeare
  :::
  ```
  - Translates to:
    ```html
    <blockquote class="quote" id="quote-id">
      <p>"To be or not to be, that is the question."</p>
      <footer>— William Shakespeare</footer>
    </blockquote>
    ```
---

#### **25. FAQ Section** BlockEnum

- **Syntax**:
  ```
  :::faq {ref: faq-id}
  - Q: What is Markdown?
    A: Markdown is a lightweight markup language for creating formatted text using a plain-text editor.
  - Q: How do I use this blog?
    A: Write Markdown content, and the converter will generate engaging HTML.
  :::
  ```
  - Translates to:
    ```html
    <div class="faq" id="faq-id">
      <div class="faq-item">
        <div class="faq-question">What is Markdown?</div>
        <div class="faq-answer">Markdown is a lightweight markup language for creating formatted text using a plain-text editor.</div>
      </div>
      <div class="faq-item">
        <div class="faq-question">How do I use this blog?</div>
        <div class="faq-answer">Write Markdown content, and the converter will generate engaging HTML.</div>
      </div>
    </div>
    ```