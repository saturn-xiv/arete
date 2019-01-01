server {
{{#ssl }}
  listen 443 ssl;
  ssl_certificate /etc/letsencrypt/live/{{name}}/fullchain.pem;
  ssl_certificate_key /etc/letsencrypt/live/{{name}}/privkey.pem;
  include /etc/letsencrypt/options-ssl-nginx.conf;
  ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;
{{/ssl}}
{{^ssl}}
  listen 80;
{{/ssl}}

  server_name {{name}};
  access_log /var/log/nginx/{{name}}.access.log;
  error_log  /var/log/nginx/{{name}}.error.log;

  location /my/ {
    alias {{root}};
    try_files $uri $uri/ /my/index.html;
  }

  location / {
    proxy_set_header X-Forwarded-Proto http{{#ssl}}s{{/ssl}};
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header Host $http_host;
    proxy_redirect off;
    proxy_pass http://localhost:{{port}};
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";
  }
}

{{#ssl}}
server {
  if ($host = {{name}}) {
    return 301 https://$host$request_uri;
  }
  listen 80;
  server_name {{name}};
  return 404;
}
{{/ssl}}
