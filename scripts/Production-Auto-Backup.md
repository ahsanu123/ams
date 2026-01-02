# Backup Production Database To Github Every Weeks

every week scheduler will auto push an update to github (make different repo for production)

# Initial Setup Explanation

- make sure current logged user is `sudo`-able user
- setup git/github authentication with [gh cli](https://cli.github.com/)
- check if `git push` is working and make commit to branch
- download latest build 
- unzip it to folder `~/ams-root/`
- then run `chmod +x auto-backup-to-github.sh`
- run this one time `loginctl enable-linger $USER` (make sure $USER is not empty)
- now copy `./ams-prod-db-backup.service` and `./ams-prod-db-backup.timer` to ` ~/.config/systemd/user/`
- then run `systemctl --user daemon-reload && systemctl --user enable --now ams-prod-db-backup.timer`
- now enable auto reboot on `06:00` and auto shutdown on `22:00`
- copy `./reboot.service` and `./reboot.timer` to `~/.config/systemd/user/`
- then run `systemctl daemon-reload && systemctl enable --now reboot.timer`
- done (you can run `./initial-setup.sh` to automatic initial setup, note you must run on extracted zip from release build)

# Test Service 

- to run service directly run `systemctl --user start ams-prod-db-backup.service`
- then check if success with `journalctl --user -u ams-prod-db-backup.service`
