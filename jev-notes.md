sudo setfacl -Rb /etc/letsencrypt

sudo setfacl -m u:atuin-server:r-x /etc/letsencrypt/archive
sudo setfacl -m u:atuin-server:r-x /etc/letsencrypt/live
sudo setfacl -m u:atuin-server:r-x /etc/letsencrypt/live/atuin.je.vin
sudo setfacl -m u:atuin-server:r-x /etc/letsencrypt/archive/atuin.je.vin
sudo setfacl -m u:atuin-server:r-- /etc/letsencrypt/archive/atuin.je.vin/*
sudo setfacl -m u:atuin-server:r-- /etc/letsencrypt/live/atuin.je.vin/*
sudo setfacl -m d:u:atuin-server:r-- /etc/letsencrypt/live/atuin.je.vin
sudo setfacl -m d:u:atuin-server:r-- /etc/letsencrypt/archive/atuin.je.vin

sudo setfacl -m u:297609:r-x /etc/letsencrypt/archive
sudo setfacl -m u:297609:r-x /etc/letsencrypt/live
sudo setfacl -m u:297609:r-x /etc/letsencrypt/live/atuin.je.vin
sudo setfacl -m u:297609:r-x /etc/letsencrypt/archive/atuin.je.vin
sudo setfacl -m u:297609:r-- /etc/letsencrypt/archive/atuin.je.vin/*
sudo setfacl -m u:297609:r-- /etc/letsencrypt/live/atuin.je.vin/*
sudo setfacl -m d:u:297609:r-- /etc/letsencrypt/live/atuin.je.vin
sudo setfacl -m d:u:297609:r-- /etc/letsencrypt/archive/atuin.je.vin
