# ronin-manager
Manager tool for ronin node

Usage:

* Initiate the `.env` file
```shell script
cp config/[test|main].env ~/.skymavis/.env
```
* Then open `.env` file and fill in remaining variables
* Start services. You might need to log out and log back in after install.
```shell script
./ronin-manager install
./ronin-manager start
```

* To stop all services
```shell script
./ronin-manager stop
```
* To stop and clean all data, including chain data and oracle data. 
Note this action is irreversible
```shell script
./ronin-manager wipe
```
