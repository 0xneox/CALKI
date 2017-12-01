#!/bin/bash
set -e

SOURCE_DIR=$(readlink -f $(dirname $0)/../..)
BINARY_DIR=${SOURCE_DIR}/target/install

################################################################################
echo -n "0) prepare  ...  "
. ${SOURCE_DIR}/tests/integrate_test/util.sh
cd ${BINARY_DIR}
echo "DONE"

################################################################################
echo -n "1) cleanup   ...  "
cleanup
echo "DONE"

################################################################################
echo -n "2) generate config  ...  "
./bin/admintool.sh > /dev/null 2>&1
echo "DONE"

################################################################################
echo -n "3) start nodes  ...  "
for i in {0..3} ; do
    bin/calki setup node$i  > /dev/null
done
for i in {0..3} ; do
    sed -i 's/"check_permission": true/"check_permission": false/g' node$i/chain.json
    bin/calki start node$i debug > /dev/null
done
echo "DONE"

################################################################################
echo -n "4) check height growth normal  ...  "
timeout=$(check_height_growth_normal 0 60)||(echo "FAILED"
                                            echo "check_height_growth_normal: ${timeout}"
                                            exit 1)
echo "${timeout}s DONE"

################################################################################
echo -n "5) create contract  ...  "
${BINARY_DIR}/bin/trans_evm --config ${SOURCE_DIR}/tests/wrk_benchmark_test/config_create.json 2>&1 | grep "sucess" > /dev/null
if [ $? -ne 0 ] ; then
    exit 1
fi
echo "DONE"

################################################################################
echo "6) send transactions continually in the background"
while [ 1 ] ; do
    ${BINARY_DIR}/bin/trans_evm --config ${SOURCE_DIR}/tests/wrk_benchmark_test/config_call.json 2>&1 >/dev/null
    sleep 1
done &
echo $! > /tmp/calki_basic-trans_evm.pid


################################################################################
echo -n "7) stop node3, check height growth  ...  "
bin/calki stop node3
timeout=$(check_height_growth_normal 0 60) || (echo "FAILED"
                                               echo "failed to check_height_growth 0: ${timeout}"
                                               exit 1)
echo "${timeout}s DONE"

################################################################################
echo -n "8) stop node2, check height stopped  ...  "
bin/calki stop node2
timeout=$(check_height_stopped 0 30) || (echo "FAILED"
                                         echo "failed to check_height_stopped 0: ${timeout}"
                                         exit 1)
echo "${timeout}s DONE"

################################################################################
echo -n "9) start node2, check height growth  ...  "
bin/calki start node2 debug
timeout=$(check_height_growth_normal 0 60) || (echo "FAILED"
                                               echo "failed to check_height_growth 0: ${timeout}"
                                               exit 1)
echo "${timeout}s DONE"

################################################################################
echo -n "10) start node3, check synch  ...  "
node0_height=$(get_height 0)

if [ $? -ne 0 ] ; then
    echo "failed to get_height: ${node0_height}"
    exit 1
fi
bin/calki start node3 debug
timeout=$(check_height_sync 3 0) || (echo "FAILED"
                                     echo "failed to check_height_synch 3 0: ${timeout}"
                                     exit 1)
echo "${timeout}s DONE"

################################################################################
echo -n "11) stop all nodes, check height changed after restart  ...  "
before_height=$(get_height 0)
if [ $? -ne 0 ] ; then
    echo "failed to get_height: ${before_height}"
    exit 1
fi
for i in {0..3}; do
    bin/calki stop node$i
done
# sleep 1 # TODO: change to this value will produce very different result
for i in {0..3}; do
    bin/calki start node$i debug
done

timeout=$(check_height_growth_normal 0 120) || (echo "FAILED"
                                               echo "failed to check_height_growth 0: ${timeout}"
                                               exit 1)
after_height=$(get_height 0)|| (echo "failed to get_height: ${after_height}"
                                exit 1)
if [ $after_height -le $before_height ]; then
    echo "FAILED"
    echo "before:${before_height} after:${after_height}"
    exit 1
fi
echo "${timeout}s DONE"

################################################################################
echo -n "12) stop&clean node3, check height synch after restart  ...  "
bin/calki stop node3
bin/calki clean node3
bin/calki start node3 debug
timeout=$(check_height_sync 3 0) || (echo "FAILED"
                                     echo "failed to check_height_synch 3 0: ${timeout}"
                                     exit 1)
echo "${timeout}s DONE"

################################################################################
echo -n "13) cleanup ... "
cleanup
echo "DONE"
