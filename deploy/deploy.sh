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
"GIT_SSH_COMMAND='ssh -i /home/kim/.ssh/id_rsa' git pull >/dev/null"
COMMIT_AFTER="$(sudo -u kim git log --pretty=format:'%h' -n 1)"

if [ "$COMMIT_BEFORE" == "$COMMIT_AFTER" ]; then
    echo "Already up to date"
else
    echo "Pulled changes from remote"
    echo "<html><p>Current commit is <code>$COMMIT_BEFORE</code></p><p>Updating to <code>$COMMIT_AFTER</code>...</p></html>" >version.html
    docker-compose build 2>build-errors.log
    if [ "$?" -eq 0 ]; then
        systemctl restart togetherness.service
        SINCE=$(systemctl status togetherness.service | grep since | sed 's/.*since //' | sed 's/;.*//')
        echo "<html><p>Current commit is <code>$COMMIT_AFTER</code></p><p>Updated from <code>$COMMIT_BEFORE</code> and restarted at $SINCE</p></html>" >version.html
    else
        echo "<html><p>Current commit is <code>$COMMIT_BEFORE</code></p><p>Attempt to update to <code>$COMMIT_AFTER</code> at $SINCE failed with errors:</p><pre>$(cat build-errors.log)</pre></html>" >version.html
    fi
fi
