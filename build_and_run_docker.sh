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

docker build . -t zion:latest

docker run -e localhost=host.docker.internal -p 8000:8000 --rm --name foo:latest -it foo:latest