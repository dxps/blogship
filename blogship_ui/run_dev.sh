#!/bin/sh

##
## Run the UI in dev mode (reloads on changes).
##
trunk serve --proxy-backend=http://[::1]:8081/api/

