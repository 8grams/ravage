#!/bin/bash

# Set default values for environment variables if not set
export AUTO_HTTPS=${AUTO_HTTPS:-"off"}

# Process Caddyfile with environment variables
envsubst < /etc/caddy/Caddyfile > /etc/caddy/Caddyfile.tmp
mv /etc/caddy/Caddyfile.tmp /etc/caddy/Caddyfile

# Start supervisor in the background
/usr/bin/supervisord -c /etc/supervisor/conf.d/supervisord.conf &

# Start Caddy in the foreground
exec caddy run --config /etc/caddy/Caddyfile