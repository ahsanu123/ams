if [ "$(tty)" = "/dev/tty1" ]; then 
	cd ~/ams-root
	sleep 5 
	exec sway
fi
