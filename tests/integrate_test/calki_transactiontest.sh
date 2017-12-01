#!/bin/bash
set +e


SOURCE_DIR=$(readlink -f $(dirname $0)/../..)
BINARY_DIR=${SOURCE_DIR}/target/install

. ${SOURCE_DIR}/tests/integrate_test/util.sh
${SOURCE_DIR}/tests/integrate_test/calki_start.sh

cd ${SOURCE_DIR}/tests/wrk_benchmark_test/
./benchmark.sh
sleep 10
./benchmark.sh config_call.json

check_height_growth 0 60

${SOURCE_DIR}/tests/integrate_test/calki_stop.sh
echo "###Test OK"
exit 0

