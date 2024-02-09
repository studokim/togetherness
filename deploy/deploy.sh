#!/usr/bin/env bash

if [ $(id -u) -ne 0 ]; then
    echo "Please run as root"
    exit 1
fi

COMMIT_BEFORE="$(git log --pretty=format:'%h' -n 1)"
sudo -u kim git pull >/dev/null
COMMIT_AFTER="$(git log --pretty=format:'%h' -n 1)"

if [ "$COMMIT_BEFORE" == "$COMMIT_AFTER" ]; then
    echo "Already up to date"
else
    echo "Pulled changes from remote"
    docker-compose build
    systemctl restart togetherness.service
    SINCE=$(systemctl status togetherness.service | grep since)
    cat <<EOF > version.html
<html>
<p>Current commit is <code>$COMMIT_AFTER</code></p>
<p>Updated from <code>$COMMIT_BEFORE</code> and restarted at $SINCE</p>
</html>
EOF
fi
