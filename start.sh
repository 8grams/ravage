#!/bin/bash

# Set default values for environment variables if not set
export APP_URL=${APP_URL:-":80"}

# Process Caddyfile with environment variables
envsubst < /etc/caddy/Caddyfile > /etc/caddy/Caddyfile.tmp
mv /etc/caddy/Caddyfile.tmp /etc/caddy/Caddyfile

# run migrations
/root/.cargo/bin/diesel migration run

# Start supervisor in the background
/usr/bin/supervisord -c /etc/supervisor/conf.d/supervisord.conf