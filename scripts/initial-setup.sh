#!/bin/bash

# run this on ~/ams-root/
cd ~/ams-root/

loginctl enable-linger $USER

cp ./ams-prod-db-backup.service ./ams-prod-db-backup.timer ~/.config/systemd/user/
cp ./reboot.service ./reboot.timer ~/.config/systemd/user/

systemctl --user daemon-reload &&
  systemctl --user enable --now ams-prod-db-backup.timer &&
  systemctl --user enable --now reboot.timer
