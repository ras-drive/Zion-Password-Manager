if [[ "$EUID" = 0 ]]; then
    echo "running as root"
else
    sudo -k # make sure to ask for password on next sudo
    if sudo true; then
        echo "running as root"
    else
        echo "wrong password"
        exit 1
    fi
fi

sudo docker build -t zion:0.1.0 .

sudo docker run zion:0.1.0