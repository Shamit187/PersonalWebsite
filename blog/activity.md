### **Initial Development Path for Blog Project**
---

#### **Phase 2: Backend Development**
-  **Render Markdown**:
  -  Integrate a Markdown-to-HTML library like Pulldown-Cmark to render blog content.
-  **Create API Endpoints**:
  -  Create an endpoint to fetch a list of blogs (titles and links).
  -  Create an endpoint to fetch full content for a specific blog by slug.

---

#### **Phase 3: Frontend Development**
-  **Create HTML Templates**:
  -  Design a homepage listing blogs with links using Tailwind CSS.
  -  Design a blog details page to display content.
-  **Add Dynamic Behavior**:
  -  Use HTMX to load blog content dynamically without refreshing the page.

---

#### **Phase 4: Web Server Configuration**
-  **Set Up NGINX**:
  -  Configure NGINX as a reverse proxy to forward requests to Axum.
  -  Set up a location for static files (e.g., Tailwind CSS).
-  **Test Locally**:
  -  Run the backend and ensure NGINX correctly forwards requests.
  -  Test serving dummy data through your frontend.

---

#### **Phase 5: Deployment**
-  **Deploy the Project**:
  -  Deploy the Axum backend to the VPS.
  -  Deploy the frontend files (if separate) to the VPS.
-  **Finalize DNS and HTTPS**:
  -  Ensure `blog.alwaysdumb.com` resolves correctly to your server.
  -  Verify HTTPS is working.
-  **Test the Website**:
  -  Open `blog.alwaysdumb.com` in a browser and test:
    -  Homepage listing dummy blogs.
    -  Clicking a link opens a blog post.
