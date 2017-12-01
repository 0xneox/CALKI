
sudo(){
    set -o noglob
    if [ "$(whoami)" == "root" ] ; then
        $*
    else
        /usr/bin/sudo $*
    fi
    set +o noglob
}


cleanup() {
    for pid in monitor jsonrpc auth chain network consensus_tendermint trans_evm ; do
        ps ax |grep ${pid}|awk '{print $1}' |xargs -n 1  kill -9 ||true
    done

    rm -rf ${BINARY_DIR}/node*
    rm -rf ${BINARY_DIR}/*.json
    sudo tc qdisc del dev lo root> /dev/null 2>&1||true

    pid_file=/tmp/calki_basic-trans_evm.pid
    if [ -e ${pid_file} ] ; then
        for pid in $(cat ${pid_file}) ; do
            kill -9 ${pid}  2>&1 || true
        done
    fi
    ps ax
}

get_height(){
    if [ $# -ne 1 ] ; then
        echo "usage: $0 node_id"
        return 1
    fi
    id=$1
    timeout=60                  # 60 seconds
    start=$(date +%s)

    while [ 1 ] ; do
        height=$(${SOURCE_DIR}/tests/integrate_test/calki_blockNumber.sh 127.0.0.1 $((1337+${id})))
        if [ $? -eq 0 ] ; then
            echo ${height}
            return 0
        fi

        now=$(date +%s)
        if [ $((now-start-timeout)) -gt 0 ] ; then
            echo "timeout: ${timeout}"
            return 1
        fi
        sleep 1
    done
    return 1
}

# output information about time used if exit 0
check_height_growth () {
    if [ $# -ne 2 ] ; then
        echo "usage: $0 node_id timeout"
        return 1
    fi
    id=$1
    timeout=$2                 # seconds
    old=$(get_height ${id})
    if [[ $? -ne 0 ]]; then
        echo "failed to get_height(old): ${old}"
        return 1
    fi
    start=$(date +%s)
    while [ 1 ] ; do
        new=$(get_height ${id})
        if [[ $? -ne 0 ]] ; then
            echo "failed to get_height(new): ${new}"
            return 1
        fi

        now=$(date +%s)
        if [ ${new} -gt ${old} ]; then
            echo "$((now-start))"
            return 0
        fi
        if [ $((now-start)) -gt ${timeout} ] ; then
            echo "timeout: $((now-start))"
            return 20
        fi
        sleep 1
    done
    return 1
}

check_height_growth_normal () {
    if [ $# -ne 2 ] ; then
        echo "usage: $0 id timeout"
        return 1
    fi

    id=$1
    timeout=$2
    start=$(date +%s)
    for i in {0..1}; do
        msg=$(check_height_growth ${id} ${timeout})
        if [ $? -ne 0 ] ; then
            echo "failed to check_height_growth ${id} ${timeout}: ${msg}"
            return 1
        fi
        if [[ ${msg} -lt ${timeout} ]]; then
            now=$(date +%s)
            echo "$((now-start))"
            return 0
        fi
    done
    echo "block height growth time(${msg}) > timeout(${timeout})"
    return 1
}

# output information about time used if exit 0
check_height_sync () {
    if [ $# -ne 2 ] ; then
        echo "usage: $0 node_id refer_node_id"
        return 1
    fi
    id=$1
    refer=$2
    timeout=60                  # seconds
    refer_height=$(get_height ${refer})
    if [ $? -ne 0 ] ; then
        echo "failed to get_height(refer): ${refer_height}"
        return 1
    fi
    start=$(date +%s)

    while [ 1 ] ; do
        height=$(get_height ${id})
        if [ $? -ne 0 ] ; then
            echo "failed to get_height(sync): ${height}"
            return 1
        fi
        now=$(date +%s)
        if [ ${height} -gt ${refer_height} ]; then
            echo "$((now-start))"
            return  0
        fi

        if [ $((now-start)) -gt ${timeout} ] ; then
            echo "timeout: $((now-start))s"
            return 1
        fi
        sleep 1
    done
    return 1
}

check_height_stopped () {
    if [ $# -ne 2 ] ; then
        echo "usage: $0 node_id timeout"
        return 1
    fi
    id=$1
    timeout=$2
    old=$(get_height ${id})
    if [ $? -ne 0 ] ; then
        echo "failed to get_height(old): ${old}"
        return 1
    fi

    start=$(date +%s)
    while [ 1 ] ; do
        now=$(date +%s)
        if [ $((now-start)) -gt ${timeout} ] ; then
            echo "$((now-start))"
            return 0
        fi
        new=$(get_height ${id})
        if [ $? -ne 0 ] ; then
            echo "failed to get_height(new): ${new}"
            return 1
        fi
        if [ $new -gt $(($old + 1)) ]; then
            # if two more blocks was generated, it shows calki still reach consensus.
            echo "height change from ${old} to ${new}"
            return 1
        fi
        sleep 1
    done
    return 1
}

set_delay_at_port() {
    if [ $# -ne 2 ] ; then
        echo "usage: set_delay_at_port port delay"
        return 1
    fi
    port=$1
    delay=$2
    # TODO: need more description
    sudo tc qdisc  add dev lo root        handle  1:  prio bands 4                                         >/dev/null 2>&1 || true
    sudo tc qdisc  add dev lo parent 1:4  handle 40:  netem delay ${delay}ms                               >/dev/null 2>&1 || true
    sudo tc filter add dev lo protocol ip parent  1:0 prio 4 u32 match ip dport ${port} 0xffff flowid 1:4  >/dev/null 2>&1 || true
}
unset_delay_at_port() {
    if [ $# -ne 1 ] ; then
        echo "usage: $0 port"
        return 1
    fi
    port=$1
    #sudo tc filter del dev lo protocol ip parent  1:0 prio 4 u32 match ip dport ${port} 0xffff flowid 1:4  >/dev/null 2>&1 || true
    sudo tc qdisc del dev lo root> /dev/null 2>&1||true
}

setup_node() {
    id=$1
    ./bin/calki setup node${id}
}

start_node() {
    id=$1
    ./bin/calki start node${id} ${debug}
}

stop_node() {
    id=$1
    ./bin/calki stop node${id}
}

stop_all () {
    stop_node 0
    stop_node 1
    stop_node 2
    stop_node 3
}

start_all () {
    start_node 0
    start_node 1
    start_node 2
    start_node 3
}
