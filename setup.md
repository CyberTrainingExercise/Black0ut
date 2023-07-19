https://github.com/BrendanKapp/OperationYJ.git

cd OperationYJ/
./setup

7z x conf.zip
sudo ./netclient.sh ID

./test.sh

logout of user

run rustdesk

set new server to rustdesk.cyberrange.rit.edu

CHANGE RUSTDESK PASSWORD
RECORD RUSTDESK ID

- Netclient Setup:

See: https://netmaker.readthedocs.io/en/master/netclient.html#netclient--page-root

Commands:
curl -sL 'https://apt.netmaker.org/gpg.key' | sudo tee /etc/apt/trusted.gpg.d/netclient.asc
curl -sL 'https://apt.netmaker.org/debian.deb.txt' | sudo tee /etc/apt/sources.list.d/netclient.list
sudo apt update
sudo apt install netclient=0.17.1-0
netclient join -t eyJhcGljb25uc3RyaW5nIjoiYXBpLm5tLjEyOS0yMS0zMy0xNDQubmlwLmlvOjQ0MyIsIm5ldHdvcmsiOiJhZnJvdGMtMiIsImtleSI6IjgwNmI0OWRiOWUwNTNiN2EiLCJsb2NhbHJhbmdlIjoiIn0=

Resize screen:
xrandr -s 1920x1080

wget https://github.com/rustdesk/rustdesk/releases/download/1.1.9/rustdesk-1.1.9.deb
dkpg -i rustdesk-1.1.9.deb