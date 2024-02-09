#!/usr/bin/env bash

if [ $(id -u) -ne 0 ]; then
    echo "Please run as root"
    exit 1
fi

if pgrep -x "docker-compose build"; then
    echo Already building
    exit 0
fi

COMMIT_BEFORE="$(sudo -u kim git log --pretty=format:'%h' -n 1)"
sudo -u kim bash -c \
"git stash save && \
GIT_SSH_COMMAND='ssh -i /home/kim/.ssh/id_rsa' git pull >/dev/null && \
git stash pop"
COMMIT_AFTER="$(sudo -u kim git log --pretty=format:'%h' -n 1)"

if [ "$COMMIT_BEFORE" == "$COMMIT_AFTER" ]; then
    echo "Already up to date"
else
    echo "Pulled changes from remote"
    cat <<EOF > version.html
<html>
<p>Current commit is <code>$COMMIT_AFTER</code></p>
<p>Updating...</p>
</html>
EOF
    docker-compose build
    systemctl restart togetherness.service
    SINCE=$(systemctl status togetherness.service | grep since | sed 's/.*since //' | sed 's/;.*//')
    cat <<EOF > version.html
<html>
<p>Current commit is <code>$COMMIT_AFTER</code></p>
<p>Updated from <code>$COMMIT_BEFORE</code> and restarted at $SINCE</p>
</html>
EOF
fi
