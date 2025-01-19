# My Struggles

## Nginx setup

This is the config I'm using for tailwind 
```conf
server {
    listen 80;
    listen [::]:80;

    server_name alwaysdumb.com www.alwaysdumb.com;

    root /var/www/out;

    location / {
        try_files $uri $uri.html $uri/ =404;
    }

    access_log /var/log/nginx/alwaysdumb_access.log;
    error_log  /var/log/nginx/alwaysdumb_error warn;

    error_page 404 /404.html;
    location = /404.html {
        internal;
    }
}
```
I still don't know how to add extra pages. Need to add extra routes

## Next static file config
Up until now I am not doing much, so here's the basic config
```
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* static website export */
  output: 'export',
};

export default nextConfig;
```
Then just use `pnpm run build` to generate the `out` file with static content


### Common Instruction

- access files change ownership `chown -R www-data:www-data /var/www/out`
- access files change access `chmod -R 755 /var/www/out`
- nginx entire config `nginx -T`
- nginx check config `nginx -t`
- nginx reload server `nginx -s reload`
- linux firewall `ufw`
- nginx link config files `ln -s /etc/nginx/sites-available/frontend-alpha.config /etc/nginx/sites-enabled/`
- allow nginx all permission to nginx `ufw allow 'Nginx Full'`
- firewall rule list `ufw status numbered`
- firewall delete specific rule `ufw delete 1`
- stop nginx `systemctl stop nginx`
- get nginx status `systemctl status nginx`

### wsl routing for windows development

- netsh interface portproxy add v4tov4 listenport=<outside-port> listenaddress=0.0.0.0 connectport=<wsl-port> connectaddress=<wsl-ip>

### easy 1 commandd ssl ceritification

- https://certbot.eff.org/instructions?ws=nginx&os=pip
- sudo certbot --nginx
- If you lose control of the domain (e.g., it expires or changes ownership), you must revoke the certificate immediately. You cannot keep using the certificate once the domain isn't under your control.
- If any information in the certificate becomes incorrect (e.g., a change in your organization details or domain ownership), you must revoke it even if the certificate is still functioning correctly.
- Certificates cannot be used in systems requiring fail-safe performance, such as air traffic control, nuclear facilities, or medical systems where failure could cause harm.
- Do not use certificates to enable systems that facilitate interception of encrypted communications, even with user consent (e.g., traffic inspection systems).
- Be aware that Let’s Encrypt may publish some details of your certificates (e.g., domain name, issue/expiry date) as part of their transparency practices.
- When requesting revocation, you must follow Let’s Encrypt’s official revocation guidelines and provide an appropriate revocation reason code.
- Your agreement with Let’s Encrypt automatically terminates if you no longer have any valid certificates issued by them. You don’t need to manually cancel the agreement.
- You are expected to inspect your certificates immediately upon issuance. If there are errors, you must request revocation right away, even if the ACME client doesn't flag them.

# Blog

## System Setup

### Reverse Proxy to add service
```
server {
    server_name blog.alwaysdumb.com;

    # root /var/www/html;
    # index index.html index.htm index.nginx-debian.html;

    location / {
        proxy_pass http://127.0.0.1:3000; # Proxy to your Axum server
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection keep-alive;
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    listen [::]:443 ssl; # managed by Certbot
    listen 443 ssl; # managed by Certbot
    ssl_certificate /etc/letsencrypt/live/blog.alwaysdumb.com/fullchain.pem; # managed by Certbot
    ssl_certificate_key /etc/letsencrypt/live/blog.alwaysdumb.com/privkey.pem; # managed by Certbot
    include /etc/letsencrypt/options-ssl-nginx.conf; # managed by Certbot
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem; # managed by Certbot
}

server {
    if ($host = blog.alwaysdumb.com) {
        return 301 https://$host$request_uri;
    } # managed by Certbot

    listen 80;
    listen [::]:80;

    server_name blog.alwaysdumb.com;
    return 404; # managed by Certbot
}
```

## systemd unit file
file name `/etc/systemd/system/blog_server.service`
```
[Unit]
Description=Rust Axum Blog Server
After=network.target

[Service]
ExecStart=/root/PersonalWebsite/blog/target/release/blog
WorkingDirectory=/root/PersonalWebsite/blog
Restart=always
User=root
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```
turn the service on by `sudo systemctl start blog_server`
reload `/etc/systemd/system/blog_server.service`
enable on boot `sudo systemctl enable blog_server`
check status `sudo systemctl status blog_server`
restart/stop `sudo systemctl stop/restart blog_server`

### sqlite
- sudo apt install sqlite3 libsqlite3-dev
