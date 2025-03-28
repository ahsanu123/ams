## 🥫 Rough Step to setup docker for cross compile in archlinuxarm

- `wget http://os.archlinuxarm.org/os/ArchLinuxARM-rpi-armv7-latest.tar.gz`
- `mkdir arch-rootfs`
- `tar -xpf ArchLinuxARM-rpi-armv7-latest.tar.gz -C arch-rootfs`
- `cd arch-rootfs`
- `tar -cpf - . | docker import - archlinuxarm:latest`
- `docker run -it --rm archlinuxarm:latest /bin/bash `
- if error when docker run, try to add qemu -> `docker run --rm --privileged multiarch/qemu-user-static --reset -p yes `
- `pacman-key --init`
- `pacman-key --populate archlinuxarm`
- `pacman -Syu`
- enable current user to access sudo (default user is `alarm`)
- `sudo  pacman  -S  --needed   webkit2gtk-4.1   base-devel  curl   wget   file   openssl   appmenu-gtk-module   libappindicator-gtk3  librsvg yarn`
- `install rust and rustup `
- `install nodejs `
- `install postgresql-libs`
- after install run `rustup  default stable `
- to save docker run -> `docker ps && docker commit <container_id_or_name> my-custom-image`
