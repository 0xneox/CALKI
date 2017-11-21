#!/bin/bash
#set -x

# 0: generate Keygen public-private key
# 1: According to the configuration file config.ini netwrok generate node configuration, the calki environment dependencies, and put the directory ./deb
# 2: If there is a public network, then generate the generated public key and generated calki install directory to the public network
# 3 - 8 Multi-server deployment for one node, 8 - 12 One server for one node
# 3: If mq separate server, configure mq service
#4: Install calki environment dependent
#5: Upload calki to each service
#6: Start calki
#7: Stop calki
# 8: upload file to server
#9: execute remote command
#10: calki setup
#11: calki start
#12: calki stop

display_help()
{
    echo
    echo "usage: $0 -c config.ini -m flag -b base_path -r remote_path -u user_name -p pwd -d cmd"
    echo "option:"
    echo "-c config"
    echo
    echo "-m flag"
    echo "
    0: Generate Keygen public and private keys
     1: According to the configuration file config.ini netwrok generate node configuration, the calki environment dependencies, and put the directory ./deb
     2: If there is a public network, then generate the generated public key and generated calki install directory to the public network
      3 - 8 Multi-server deployment for one node, 8 - 12 One server for one node
     3: If mq separate server, configure mq service
     4: Install calki environment dependent
     5: Upload calki to each service
      6: Start calki
      7: Stop calki
     8: upload files to the server 9: execute remote commands 10: calki settings 11: calki start 12: calki stop
         "
    echo
    echo "-b base_path(bin base directory)"
    echo
    echo "-r remote_path"
    echo
    echo "-u user_name"
    echo
    echo "-p pwd"
    echo
    echo "-d cmd"
    echo
    echo
    exit 0
}


# Read the configuration file
function readINIfile()
{
    Key=$1
    Section=$2
    Configfile=$3
    ReadINI=`awk -F '=' '/\['$Section'\]/{a=1}a==1&&$1~/'$Key'/{print $2;exit}' $Configfile`
    echo "$ReadINI"
}

# Generate public and private keys
function Keygen()
{
expect <<-EOF
set timeout 5

spawn ssh-keygen -t rsa
expect {
    "Enter file in which to save the key" { send "\r"; exp_continue }
    "Enter passphrase (empty for no passphrase):" { send "\r"; exp_continue }
    "Enter same passphrase again:" { send "\r" }
}
expect EOF ;
EOF
}

# Upload the public key, we must first create. ssh directory
function remote_mkdir()
{
dst_host=$1
dst_username=$2
dst_passwd=$3
expect <<-EOF
spawn ssh $dst_username@$dst_host "mkdir -p ~/.ssh > /dev/null"
expect {
    "(yes/no)" { send "yes\r"; exp_continue }
    "password:" { send "$dst_passwd\r" }
}
set timeout 30;
send "exit\r"
expect EOF ;
EOF
}

function scp_pub_to_remote()
{
ip=$1
user_name=$2
password=$3
src_file=~/.ssh/id_rsa.pub
dest_file="~/.ssh/authorized_keys"

expect <<-EOF
set timeout -1
spawn scp  "$src_file" $user_name@$ip:$dest_file
expect {
    "(yes/no)" { send "yes\r"; exp_continue }
    "password:" { send "$password\r" }
}
expect "100%"
expect EOF ;
EOF
}

#apt Download dependencies and put in a fixed directory
function apt_deb()
{
    deb_path="/var/cache/apt/archives/"
    # libgoogle-perftools libunwind (required for each process)
     # libsodium  (chain needs)
    #rabbitmq-server
    sudo apt-get -y -d --reinstall  install rabbitmq-server libsodium* libgoogle-perftools-dev google-perftools libunwind8 libunwind8-dev libunwind-dev libltdl7 libodbc1 libgoogle-perftools4 libtcmalloc-minimal4 erlang
    mkdir -p deb
    find $deb_path -maxdepth 1 -name "rabbitmq-server*" -exec cp {} ./deb \;
    find $deb_path -maxdepth 1 -name "libsodium*" -exec cp {} ./deb \;
    find $deb_path -maxdepth 1 -name "libgoogle-perftools*" -exec cp {} ./deb \;
    find $deb_path -maxdepth 1 -name "libunwind*" -exec cp {} ./deb \;
    find $deb_path -maxdepth 1 -name "libltdl7*" -exec cp {} ./deb \;
    find $deb_path -maxdepth 1 -name "libodbc1*" -exec cp {} ./deb \;
    find $deb_path -maxdepth 1 -name "libgoogle-perftools4*" -exec cp {} ./deb \;
    find $deb_path -maxdepth 1 -name "libtcmalloc-minimal4*" -exec cp {} ./deb \;
    find $deb_path -maxdepth 1 -name "erlang*" -exec cp {} ./deb \;
}

# Download libgmssl
function wget_libgmssl()
{
    mkdir -p deb
    cd deb
    wget https://github.com/cryptape/GmSSL/releases/download/v1.0/libgmssl.so.1.0.0.gz
    gzip -d libgmssl.so.1.0.0.gz
    cd ..
}
#upload files
function scp_file_to_remote()
{
ip=$1
user_name=$2
src_file=$3
dest_file=$4
scp -r $src_file $user_name@$ip:$dest_file
}

# Execute the server command
function remote_run_cmd()
{
ip=$1
user_name=$2
cmd=$3
ssh $user_name@$ip "$cmd"
}

# Start calki
function calki_start()
{
    node=node$1
    user_name=$2
    auth_host=$3
    network_host=$4
    consensus_host=$5
    jsonrpc_host=$6
    chain_host=$7
    echo "starting ${node}"
    ssh $user_name@$auth_host      "cd install/${node}; mkdir -p logs;nohup ../bin/auth                                       >logs/${node}.auth       2>&1 & echo $! >> .pid"
    ssh $user_name@$network_host   "cd install/${node}; mkdir -p logs;nohup ../bin/network                 -c network.toml    >logs/${node}.network    2>&1 & echo $! >> .pid"
    ssh $user_name@$consensus_host "cd install/${node}; mkdir -p logs;nohup ../bin/consensus_tendermint    -c consensus.json  >logs/${node}.consensus  2>&1 & echo $! >> .pid"
    ssh $user_name@$jsonrpc_host   "cd install/${node}; mkdir -p logs;nohup ../bin/jsonrpc                 -c jsonrpc.json    >logs/${node}.jsonrpc    2>&1 & echo $! >> .pid"
    ssh $user_name@$chain_host     "cd install/${node}; mkdir -p logs;nohup ../bin/chain  -g genesis.json  -c chain.json      >logs/${node}.chain      2>&1 & echo $! >> .pid"
}

#Stop calki
function calki_stop()
{
    node=node$1
    user_name=$2
    auth_host=$3
    network_host=$4
    consensus_host=$5
    jsonrpc_host=$6
    chain_host=$7
    echo "stop ${node}"
    ssh $user_name@$auth_host      "killall auth;cd install/${node}; rm -rf data/*"
    ssh $user_name@$network_host   "killall network; cd install/${node}; rm -rf data/*"
    ssh $user_name@$consensus_host "killall consensus_tendermint; cd install/${node}; rm -rf data/*"
    ssh $user_name@$jsonrpc_host   "killall jsonrpc; cd install/${node}; rm -rf data/*"
    ssh $user_name@$chain_host     "killall chain; cd install/${node}; rm -rf data/*"
}


CUR_PATH=$(cd `dirname $0`; pwd)
# parse options usage: $0 -c config.ini -m flag -b base_path -r remote_path -u user_name -p pwd -t
while getopts 'c:m:b:r:u:p:d:' OPT; do
    case $OPT in
        c)
            config="$OPTARG";;
        m)
            method="$OPTARG";;
        b)
            base_path="$OPTARG";;
        r)
            remote_path="$OPTARG";;
        u)
            user_name="$OPTARG";;
        p)
            pwd="$OPTARG";;
        d)
            cmd="$OPTARG";;
        ?)
            display_help
    esac
done

#set default value
if [ ! -n "$config" ]; then
    config="config.ini"
fi

if [ ! -n "$method" ]; then
    echo "method must be set up"
    exit 0
fi

if [ ! -n "$base_path" ]; then
    echo "base_path must be set up"
    exit 0
fi

if [ ! -n "$remote_path" ]; then
    remote_path="~/"
fi

if [ ! -n "$user_name" ]; then
    echo "user_name must be set up"
    exit 0
fi

if [ ! -n "$pwd" ]; then
    if [ $method -eq 2 ]; then
        echo "pwd must be set up"
        exit 0
    fi
    pwd=""
fi

if [ ! -n "$cmd" ]; then
    cmd=""
fi

node_num=4

if [ $method -eq 0 ]; then
  # Generate public and private keys
    rm -rf ~/.ssh
    Keygen > /dev/null

elif [ $method -eq 1 ]; then
  # Generate the configuration of the node
    admintool_path=$base_path
    echo "admintool_path = "$admintool_path
    # Read the configuration file
    network_host=`readINIfile "netwrok" "host" "$config"`

    admintool="./bin/admintool.sh -l "$network_host
    echo "执行: "$admintool
    cd $admintool_path
    $($admintool > /dev/null 2>&1)
    cd $CUR_PATH

    # Download deb
    apt_deb

elif [ $method -eq 2 ]; then
    remote_host=`readINIfile "remote_host" "host" "$config"`
    echo "$pwd"
    # Upload the public key to the public network
    scp_pub_to_remote $remote_host $user_name "$pwd"

    # Upload calki execution file to the external network server
    src_calki="$base_path"
    scp_file_to_remote $remote_host $user_name "$src_calki" "$remote_path"

elif [ $method -eq 3 ]; then
  #mq server configuration
     echo "mq server configuration"
    amqp_host=`readINIfile "amqp" "host" "$config"`
    OLD_IFS="$IFS"
    IFS=":"
    amqp_host_arr=($amqp_host)
    IFS="$OLD_IFS"
    length=${#amqp_host_arr[@]}
    i=0
    install_cmd="dpkg -i rabbitmq-server*"
    rabbitmq_path="deb/rabbitmq-server*"
    while :
    do
      # mq server upload public key
        remote_mkdir ${amqp_host_arr[$i]} $user_name $pwd
        scp_pub_to_remote ${amqp_host_arr[$i]} $user_name $pwd > /dev/null

      # Upload deb to mq service
        scp_file_to_remote "$remote_host" "$user_name" "$rabbitmq_path" "$remote_path"

        # dpkg -i installation
        remote_run_cmd ${amqp_host_arr[$i]} $user_name "$install_cmd"

        # Generate rabbitmq user
        add_vhost_cmd="rabbitmqctl add_vhost node$i"
        remote_run_cmd ${amqp_host_arr[$i]} $user_name "$add_vhost_cmd"

        add_user_cmd="rabbitmqctl add_user calki calki"
        remote_run_cmd ${amqp_host_arr[$i]} $user_name "$add_user_cmd"

        set_user_tags_cmd="rabbitmqctl set_user_tags calki  administrator"
        remote_run_cmd ${amqp_host_arr[$i]} $user_name "$set_user_tags_cmd"

        set_permissions_cmd='rabbitmqctl set_permissions -p node$i calki ".*" ".*" ".*"'
        remote_run_cmd ${amqp_host_arr[$i]} $user_name "$set_permissions_cmd"

        service_cmd="service rabbitmq-server restart"
        remote_run_cmd ${amqp_host_arr[$i]} $user_name "$service_cmd"

        i=$[$i+1]
        if [ $i -eq $length ]; then
            break
        fi
    done

elif [ $method -eq 4 ]; then

  # Install calki environment dependecies
    amqp_host=`readINIfile "amqp" "host" "$config"`
    OLD_IFS="$IFS"
    IFS=":"
    amqp_host_arr=($amqp_host)
    IFS="$OLD_IFS"
    length=${#amqp_host_arr[@]}
    deb_install="cd deb;dpkg -i libsodium* libgoogle-perftools* libunwind*"
    for((i=0; i<$node_num; i++))
    do
        Section="node""$i"
        host_name=$Section"_mq"
        # Modify each node's .env
        sed -ig "s/localhost/$host_name/g" $base_path/node$i/.env
        sed -ig "s/guest/calki/g" $base_path/node$i/.env

        cmd="echo ${amqp_host_arr[$i]}    $host_name >> /etc/hosts"
        #jsonrpc
        jsonrpc_host=`readINIfile "jsonrpc" "$Section" "$config"`
        remote_mkdir $jsonrpc_host $user_name $pwd
        scp_pub_to_remote $jsonrpc_host $user_name > /dev/null
        remote_run_cmd "$jsonrpc_host" "$user_name" "$cmd"
      # Upload deb to the service and install
        scp_file_to_remote "$jsonrpc_host" "$user_name" "deb" "~/"
        remote_run_cmd "$jsonrpc_host" "$user_name" "$deb_install"

        #chain
        chain_host=`readINIfile "chain" "$Section" "$config"`
        remote_mkdir $chain_host $user_name $pwd
        scp_pub_to_remote $chain_host $user_name $pwd > /dev/null
        remote_run_cmd "$chain_host" "$user_name" "$cmd"
        # Upload deb to the service and install
        scp_file_to_remote "$chain_host" "$user_name" "deb" "~/"
        remote_run_cmd "$chain_host" "$user_name" "$deb_install"

        #consensus
        consensus_host=`readINIfile "consensus" "$Section" "$config"`
        remote_mkdir $consensus_host $user_name $pwd
        scp_pub_to_remote $consensus_host $user_name $pwd > /dev/null
        remote_run_cmd "$consensus_host" "$user_name" "$cmd"
        # Upload deb to the service and install
        scp_file_to_remote "$consensus_host" "$user_name" "deb" "~/"
        remote_run_cmd "$consensus_host" "$user_name" "$deb_install"

        #auth
        auth_host=`readINIfile "auth" "$Section" "$config"`
        remote_mkdir $auth_host $user_name $pwd
        scp_pub_to_remote $auth_host $user_name $pwd > /dev/null
        remote_run_cmd "$auth_host" "$user_name" "$cmd"
        # Upload deb to the service and install
        scp_file_to_remote "$auth_host" "$user_name" "deb" "~/"
        remote_run_cmd "$auth_host" "$user_name" "$deb_install"
    done

elif [ $method -eq 5 ]; then
# Upload calki to each service
    src_calki="$base_path"
    for((i=0; i<$node_num; i++))
    do
        Section="node""$i"
        host_name=$Section"_mq"
        echo "=============$Section================"
        # Modify each node's .env
        sed -ig "s/localhost/$host_name/g" $src_calki/node$i/.env
        sed -ig "s/guest/calki/g" $src_calki/node$i/.env
        #jsonrpc
        jsonrpc_host=`readINIfile "jsonrpc" "$Section" "$config"`
        ssh $user_name@$jsonrpc_host "rm -rf install"
        scp_file_to_remote $jsonrpc_host $user_name $src_calki "$remote_path"
        #chain
        chain_host=`readINIfile "chain" "$Section" "$config"`
        ssh $user_name@$chain_host "rm -rf install"
        scp_file_to_remote $chain_host $user_name $src_calki "$remote_path"
        #consensus
        consensus_host=`readINIfile "consensus" "$Section" "$config"`
        ssh $user_name@$consensus_host "rm -rf install"
        scp_file_to_remote $consensus_host $user_name $src_calki "$remote_path"
        #auth
        auth_host=`readINIfile "auth" "$Section" "$config"`
        ssh $user_name@$auth_host "rm -rf install"
        scp_file_to_remote $auth_host $user_name $src_calki "$remote_path"
    done



elif [ $method -eq 6 ]; then
# Start calki
    for((i=0; i<$node_num; i++))
    do
        Section="node""$i"
        #jsonrpc
        jsonrpc_host=`readINIfile "jsonrpc" "$Section" "$config"`
        #chain
        chain_host=`readINIfile "chain" "$Section" "$config"`
        #consensus
        consensus_host=`readINIfile "consensus" "$Section" "$config"`
        #auth
        auth_host=`readINIfile "auth" "$Section" "$config"`

        calki_start $i $user_name $auth_host $consensus_host $consensus_host $jsonrpc_host $chain_host
    done
elif [ $method -eq 7 ]; then
#stop calki
    for((i=0; i<0; i++))
    do
        Section="node""$i"
        #jsonrpc
        jsonrpc_host=`readINIfile "jsonrpc" "$Section" "$config"`
        #chain
        chain_host=`readINIfile "chain" "$Section" "$config"`
        #consensus
        consensus_host=`readINIfile "consensus" "$Section" "$config"`
        #auth
        auth_host=`readINIfile "auth" "$Section" "$config"`
        calki_stop $i $user_name $auth_host $consensus_host $consensus_host $jsonrpc_host $chain_host
    done

elif [ $method -eq 8 ]; then
# Upload files to service
    calki_host=`readINIfile "calki" "host" "$config"`
    OLD_IFS="$IFS"
    IFS=":"
    calki_host_arr=($calki_host)
    IFS="$OLD_IFS"
    length=${#calki_host_arr[@]}
    i=0
    while :
    do
        # mq server upload public key
        remote_mkdir ${calki_host_arr[$i]} $user_name $pwd
        scp_pub_to_remote ${calki_host_arr[$i]} $user_name $pwd > /dev/null
        scp_file_to_remote "${calki_host_arr[$i]}" "$user_name" "$base_path" "$remote_path"

        i=$[$i+1]
        if [ $i -eq $length ]; then
            break
        fi
    done
elif [ $method -eq 9 ]; then
#Executing an order
    calki_host=`readINIfile "calki" "host" "$config"`
    OLD_IFS="$IFS"
    IFS=":"
    calki_host_arr=($calki_host)
    IFS="$OLD_IFS"
    length=${#calki_host_arr[@]}
    i=0
    while :
    do
        remote_run_cmd "${calki_host_arr[$i]}" "$user_name" "$cmd"
        i=$[$i+1]
        if [ $i -eq $length ]; then
            break
        fi
    done
elif [ $method -eq 10 ]; then
# Use the calki command setup
    echo "calki setup"
    calki_host=`readINIfile "calki" "host" "$config"`
    OLD_IFS="$IFS"
    IFS=":"
    calki_host_arr=($calki_host)
    IFS="$OLD_IFS"
    length=${#calki_host_arr[@]}
    i=0
    while :
    do
        remote_run_cmd ${calki_host_arr[$i]} $user_name "cd $base_path;rm -rf node$i/data;./bin/calki setup node$i" &
        i=$[$i+1]
        if [ $i -eq $length ]; then
            break
        fi
    done
elif [ $method -eq 11 ]; then
  # Start calki with the calki command
       echo "start calki"
    calki_host=`readINIfile "calki" "host" "$config"`
    OLD_IFS="$IFS"
    IFS=":"
    calki_host_arr=($calki_host)
    IFS="$OLD_IFS"
    length=${#calki_host_arr[@]}
    i=0
    while :
    do
        # mq server upload public key
        remote_run_cmd ${calki_host_arr[$i]} $user_name "cd $base_path;./bin/calki start node$i" &

        i=$[$i+1]
        if [ $i -eq $length ]; then
            break
        fi
    done
elif [ $method -eq 12 ]; then
# Stop calki with the calki command
    echo "停止calki"
    calki_host=`readINIfile "calki" "host" "$config"`
    OLD_IFS="$IFS"
    IFS=":"
    calki_host_arr=($calki_host)
    IFS="$OLD_IFS"
    length=${#calki_host_arr[@]}
    i=0
    while :
    do
        # mq server upload public key
        remote_run_cmd ${calki_host_arr[$i]} $user_name "cd install;./bin/calki stop node$i;rm -rf node$i/data" &

        i=$[$i+1]
        if [ $i -eq $length ]; then
            break
        fi
    done
fi
