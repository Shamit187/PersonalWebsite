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