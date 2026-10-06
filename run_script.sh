
$ cat /mnt/mtd/ipc/conf/run
#! /bin/sh

TARGET="/mnt/mtd/ipc"
CONF="$TARGET/conf"
NETINFO=$CONF/config_net.ini
NETPRIV=$CONF/config_priv.ini
PLATFORM=$CONF/config_platform.ini
NETDEV=eth0
WIFIST=0
WIFIPATH="$CONF/wifi.conf"
. $WIFIPATH

DEF_IPADDR="192.168.1.88"
DEF_GATEWAY="192.168.1.1"

closeLed()
{
        echo 10 > /sys/class/gpio/export
        echo out > /sys/class/gpio/gpio10/direction
        echo 0 > /sys/class/gpio/gpio10/value
}

#close audio out
closeAudioout()
{
        echo 13 > /sys/class/gpio/export
        echo out > /sys/class/gpio/gpio13/direction
        echo 1 > /sys/class/gpio/gpio13/value
}

clearRoute()
{
        GATEWAY=`route -n | grep UG | awk -F " " '{printf $2}'`
        route del default gw $GATEWAY
        GIP=`route -n | grep eth0 | awk -F " " '{printf $1}'`
        GNM=`route -n | grep eth0 | awk -F " " '{printf $3}'`
        route del -net $GIP netmask $GNM
}

setRoute()
{
        ipaddr1=`ifconfig $NETDEV | grep "inet addr:" | awk '{printf $2}' | awk -F ":" '{printf $2}'`
        gat1=`echo $ipaddr1 | awk -F "." '{print $1}'`
        gat2=`echo $ipaddr1 | awk -F "." '{print $2}'`
        gat3=`echo $ipaddr1 | awk -F "." '{print $3}'`
        routeway1="$gat1.$gat2.$gat3.0" 
        route add -net $routeway1 netmask 255.255.255.0 $NETDEV
}

wpa_run()
{
        echo "ctrl_interface=/var/run/wpa_supplicant" > /mnt/mtd/ipc/tmpfs/wpa.conf
        echo "update_config=1" >> /mnt/mtd/ipc/tmpfs/wpa.conf
        echo "network={" >> /mnt/mtd/ipc/tmpfs/wpa.conf
        echo "    ssid=\"asdfiwedfj#!@\"" >> /mnt/mtd/ipc/tmpfs/wpa.conf
        echo "    key_mgmt=NONE" >> /mnt/mtd/ipc/tmpfs/wpa.conf
        echo "    auth_alg=OPEN" >> /mnt/mtd/ipc/tmpfs/wpa.conf
        echo "    priority=r" >> /mnt/mtd/ipc/tmpfs/wpa.conf
        echo "}" >> /mnt/mtd/ipc/tmpfs/wpa.conf
        echo "" >> /mnt/mtd/ipc/tmpfs/wpa.conf  
        wpa_supplicant -B -D wext -i wlan0  -c /mnt/mtd/ipc/tmpfs/wpa.conf
        wpa_cli -i wlan0 -p /var/run/wpa_supplicant remove_network 0
}

getipcmac()
{
    DEVMAC="wlan0"
    if ! ifconfig $DEVMAC up
        then    
                WIFIST=0        
        echo "00:00:00:00:00:00" > $TARGET/tmpfs/wifi.mac
                return 0
        fi
    $TARGET/getdevmac $DEVMAC > $TARGET/tmpfs/wifi.mac
    WIFIST=1
    wpa_run
}
        
loadwifista()
{
        wpa_cli -i wlan0 -p /var/run/wpa_supplicant remove_network 0
        wpa_cli -i wlan0 -p /var/run/wpa_supplicant add_network
        wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 scan_ssid 1
        wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 ssid "\"${WifiSsid}\""
        if [ $WifiEnc == "NONE" ]
        then
                wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 key_mgmt NONE
        elif [ $WifiEnc == "WEP" ]
        then    
                wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 key_mgmt NONE
                StrNum=`echo "$WifiKey" | wc -L`
                if [ $StrNum -eq  5 ]
                then    
                        wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 wep_key0 "\"$WifiKey\""
                elif [ $StrNum -eq  13 ]
                then
                        wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 wep_key0 "\"$WifiKey\""
                else
                        wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 wep_key0 $WifiKey
                fi
                wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 wep_tx_keyidx 0
                wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 auth_alg 'OPEN SHARED'
        else
                wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 key_mgmt WPA-PSK
                wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 psk "\"$WifiKey\""
        fi
        wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 scan_ssid 1
        wpa_cli -i wlan0 -p /var/run/wpa_supplicant select_network 0
}

fixnet()
{
        unzip -o $TARGET/config_default.zip -d $TARGET/tmpfs/
        cp $TARGET/tmpfs/mnt/mtd/ipc/conf/config_net.ini $TARGET/conf
        sync
        reboot
}

loadwifiap()
{
        TUUID=`/mnt/mtd/ipc/readcfg $PLATFORM "xquncfg:xqunuuid"`
        TUUID=`echo $TUUID | awk -F "-" '{print $2}'`
        WifiSsid="IPCAM-$TUUID"
        TMP=/mnt/mtd/ipc/tmpfs/wf129
        TMP1=/mnt/mtd/ipc/tmpfs/wf129t
        sleep 1
        iwlist wlan0 scanning > /dev/null
        sleep 1
        iwlist wlan0 scanning > /dev/null
        sleep 1
        iwlist wlan0 scanning > /dev/null
        wpa_cli -p/var/run/wpa_supplicant -i wlan0 scan_result > $TMP
        /mnt/mtd/ipc/wfsort $TMP $TMP1
        mv $TMP1 $TMP
        wpa_cli terminate
        sleep 1
        wpa_cli terminate
        ifconfig wlan0 $DEF_IPADDR netmask 255.255.255.0
        cp $TARGET/conf/hostapd8188.conf $TARGET/tmpfs/hostapd.conf
        echo "ssid=$WifiSsid" >> $TARGET/tmpfs/hostapd.conf
        echo channel=$(($RANDOM%10+1)) >> $TARGET/tmpfs/hostapd.conf
        hostapd $TARGET/tmpfs/hostapd.conf -B
        udhcpd /mnt/mtd/ipc/conf/udhcps/udhcpd.conf
        route add default gw $DEF_GATEWAY
}

loadnet()
{
    dhcp1=`grep dhcp $NETINFO | awk -F "\"" '{print $2}'`
    ipaddr1=`grep ipaddr $NETINFO | awk -F "\"" '{print $2}'`
    gateway1=`grep gateway $NETINFO | awk -F "\"" '{print $2}'`
    netmask1=`grep netmask $NETINFO | awk -F "\"" '{print $2}'` 

    gat1=`echo $ipaddr1 | awk -F "." '{print $1}'`
    gat2=`echo $ipaddr1 | awk -F "." '{print $2}'`
    gat3=`echo $ipaddr1 | awk -F "." '{print $3}'`
    gat4=`echo $ipaddr1 | awk -F "." '{print $4}'`
    gateway2="$gat1.$gat2.$gat3.1"              
                
        if [ $dhcp1 = "y" ] 
        then
                $TARGET/dhcp.sh $NETDEV &
                return 0
        fi   

        ipnum=`echo "$ipaddr1" | wc -L`
        if [ $ipnum -eq 0 ]
        then
                fixnet
        elif ! ifconfig $NETDEV $ipaddr1 netmask $netmask1
        then
                fixnet  
        fi
        
        if ! route add default gw $gateway1
        then
                if ! route add default gw $gateway2
                then
                        route add default gw $DEF_GATEWAY
                fi
        fi
                
        $TARGET/runarp $NETDEV & > /dev/null    
                
        return 0      
}

initNet()
{
SD_WFFLAG="gcardinit2014.dat"

        if ! cat /proc/partitions  | grep mmcblk0p1
        then
                return 1
        fi
        
        mount /dev/mmcblk0p1 $TARGET/tmpfs/sd
        if ! ls $TARGET/tmpfs/sd/$SD_WFFLAG
        then
                umount $TARGET/tmpfs/sd
                return 1
        fi

        cp $TARGET/tmpfs/sd/$SD_WFFLAG $TARGET/tmpfs
        umount $TARGET/tmpfs/sd
        
. $TARGET/tmpfs/$SD_WFFLAG

        wpa_cli -i wlan0 -p /var/run/wpa_supplicant remove_network 0
        wpa_cli -i wlan0 -p /var/run/wpa_supplicant add_network
        wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 scan_ssid 1
        wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 ssid "\"${iWifiSsid}\""
        if [ $iWifiEnc == "NONE" ]
        then
                wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 key_mgmt NONE
        elif [ $iWifiEnc == "WEP" ]
        then    
                wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 key_mgmt NONE
                StrNum=`echo "$iWifiKey" | wc -L`
                if [ $StrNum -eq  5 ]
                then    
                        wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 wep_key0 "\"$iWifiKey\""
                elif [ $StrNum -eq  13 ]
                then
                        wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 wep_key0 "\"$iWifiKey\""
                else
                        wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 wep_key0 $iWifiKey
                fi
                wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 wep_tx_keyidx 0
                wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 auth_alg 'OPEN SHARED'
        else
                wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 key_mgmt WPA-PSK
                wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 psk "\"$iWifiKey\""
        fi
        wpa_cli -i wlan0 -p /var/run/wpa_supplicant set_network 0 scan_ssid 1
        wpa_cli -i wlan0 -p /var/run/wpa_supplicant select_network 0

    ipaddr1=$iIPAddr
    gat1=`echo $ipaddr1 | awk -F "." '{print $1}'`
    gat2=`echo $ipaddr1 | awk -F "." '{print $2}'`
    gat3=`echo $ipaddr1 | awk -F "." '{print $3}'`
    gat4=`echo $ipaddr1 | awk -F "." '{print $4}'`
    gateway2="$gat1.$gat2.$gat3.1"      
        ifconfig wlan0 $ipaddr1 netmask 255.255.255.0
        route add default gw $gateway2
        $TARGET/runarp wlan0 & > /dev/null
        touch $TARGET/tmpfs/wifi.sd
    
    $TARGET/updatewifi 20 & 
    
    return 0
}

loadNetwork()
{
        echo "0" > $TARGET/tmpfs/netflag.dat
        hostname IPCamera
        NETDEV=eth0     

        $TARGET/setnet  
        sleep 3
        
        if [ $WifiEnable -eq 0 ]
        then
                loadnet
                return 0
        fi

        if [ $WIFIST -eq 0 ]
        then
                loadnet
                return 0
        fi      

        if $TARGET/chknet
        then    
                loadnet
                return 0
        fi

        echo "1" > $TARGET/tmpfs/netflag.dat
        NETDEV=wlan0
        clearRoute
        setRoute
        if [ $WifiType = "Infra" ]      
        then
                loadwifista
                $TARGET/updatewifi 15 &
                loadnet
        elif [ $WifiType = "Adhoc" ]
        then
                loadwifiap
        else
                if initNet
                then
                        return 0
                fi
                ifconfig $NETDEV $DEF_IPADDR netmask 255.255.255.0
                route add default gw $DEF_GATEWAY
        fi
}       

linkFile()
{
        mkdir $TARGET/tmpfs/sd
        mkdir $TARGET/tmpfs/var
        mkdir /mnt/mtd/ipc/tmpfs/lib
        mkdir /var/run
        mkdir /var/lib/misc/ -p
        touch /var/lib/misc/udhcpd.leases
}

loadAround()
{
    $TARGET/facddns.sh
    $TARGET/upnpmap.sh
    $TARGET/th3ddns.sh
}

chkdebug()
{
        dtenable=`grep tenable $CONF/config_debug.ini | awk -F "\"" '{print $2}'`
        if [ $dtenable -eq 1 ]
        then
                telnetd
        fi
}

executevs()
{
        unzip -o $TARGET/conf/ipc_server -d $TARGET/tmpfs/ || cp $TARGET/ipc_server -d $TARGET/tmpfs/
        chmod a+x $TARGET/tmpfs/ipc_server
        unzip -o $TARGET/conf/libNetLib.so -d $TARGET/tmpfs/ || cp $TARGET/libNetLib.so -d $TARGET/tmpfs/
        unzip -o $TARGET/conf/libXqAPILib.so -d $TARGET/tmpfs/ || cp $TARGET/libXqAPILib.so -d $TARGET/tmpfs/
        unzip -o $TARGET/conf/libxqun.so -d $TARGET/tmpfs/ || cp $TARGET/libxqun.so -d $TARGET/tmpfs/
}

loadmotor()
{
        DEVTYPE=$CONF/config_devtype.ini
        MOTORF=`grep motormode $DEVTYPE | awk -F "\"" '{print $2}'`
        PANS=`grep panrange $DEVTYPE | awk -F "\"" '{print $2}'`
        TILS=`grep tiltrange $DEVTYPE | awk -F "\"" '{print $2}'`
        insmod $TARGET/modules/motor.ko pan_all=$PANS titl_all=$TILS spd_all=1100 spd_pan=1 spd_titl=1
}

setsysa()
{
        echo "2" > /proc/sys/net/ipv4/conf/all/force_igmp_version
        echo 1536 > /proc/sys/vm/min_free_kbytes
        echo 1 > /proc/sys/vm/overcommit_memory  
}

chksen_wifi()
{
        TMP=/mnt/mtd/ipc/tmpfs/wf129
        TMP1=/mnt/mtd/ipc/tmpfs/wf129t

        if grep "sensor=-1" /mnt/mtd/ipc/tmpfs/sensor.conf
        then
                iwlist wlan0 scanning > /dev/null
                wpa_cli -p/var/run/wpa_supplicant -i wlan0 scan_result > $TMP
                /mnt/mtd/ipc/wfsort $TMP $TMP1
                mv $TMP1 $TMP
        fi
}

closeAudioout
closeLed

mount -t usbfs /proc/bus/usb /proc/bus/usb
mount tmpfs $TARGET/tmpfs -t tmpfs -o size=16m

linkFile

$TARGET/sd_first

executevs

/mnt/mtd/ipc/watchdog &
$TARGET/load_drv
$TARGET/load_media

getipcmac

loadmotor

loadNetwork

chkdebug

setsysa

$TARGET/watchdog &

$TARGET/sd.sh &

$TARGET/setlinuxtime 2017.08.29-04:10:11

loadAround

$TARGET/platform.sh &

$TARGET/chksock &

$TARGET/net_detect &

chksen_wifi

$TARGET/tmpfs/ipc_server &
$ 