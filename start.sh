#!/bin/sh

/root/.cargo/bin/diesel migration run

/usr/bin/supervisord -c /etc/supervisord.conf