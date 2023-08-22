#!/bin/bash

#set -x
set -e
set -u

DFLT_CPUS=1
DFLT_INTERVAL=60
# At the moment this is the only supported test
DFLT_TARGET=transaction
DFLT_AFL_TIMEOUT=1000

function usage() {
    echo "$0 [COMMAND] [FUZZ-TARGET] [COMMAND-ARGS]"
    echo "Available targets:"
    echo "    transaction"
    echo "    wasm_instrument"
    echo "Commands:"
    echo "    run <target ><duration> <instances> [timeout]"
    echo "            Run given number of AFL instances (default: $DFLT_CPUS) in screen sessions"
    echo "            for a given number of seconds."
    echo "            For 'instances' one can specify"
    echo "              all      - to run as many instances as CPU cores available"
    echo "              <number> - to run <number> of instances"
    echo "            'timeout' is an AFL timeout in ms"
    echo "    watch <target> <interval>"
    echo "            Monitor AFL instances until they are finished."
    echo "            One can specify interval (default: $DFLT_INTERVAL) to output the status"
    echo "    quit <target>   Quit all AFL instances"
}

function error() {
    local msg=$1
    echo "error - $msg"
    usage
    exit 1
}

function get_cpus() {
    local uname="$(uname -s)"
    if [ $uname = "Linux" ] ; then
        cat /proc/cpuinfo  | grep processor | wc -l
    elif [ $uname = "Darwin" ] ; then
        sysctl -n hw.ncpu
    else
        echo "OS $uname not supported"
        exit 1
    fi
}

function humanize_seconds()
{
   local t=$1
   local d=$((t / 60 / 60 / 24))
   local h=$((t / 60 / 60 % 24))
   local m=$((t / 60 % 60))
   local s=$((t % 60))

   if [ $d -ne 0 ] ; then
      printf '%d days %02d hours %02d minutes %02d seconds' $d $h $m $s
   else
      printf '%02d hours %02d minutes %02d seconds' $h $m $s
   fi
}

cmd=watch
if [ $# -ge 1 ] ; then
    cmd=$1
    shift
fi
target=$DFLT_TARGET
if [ $# -ge 1 ] ; then
    target=$1
    shift
fi

# Trying different power schedules, but keeping in mind that "fast" and "explore" are most effective ones,
# thus they are duplicated.
# The more CPU cores available, the more schedules possible to try.
# More details on power schedules:
#   https://github.com/AFLplusplus/AFLplusplus/blob/stable/docs/fuzzing_in_depth.md#c-using-multiple-cores
#   https://github.com/AFLplusplus/AFLplusplus/blob/stable/docs/FAQ.md#what-are-power-schedules
SCHEDULES_ARR=("fast" "explore" "fast" "explore" "exploit" "coe" "lin" "quad" "seek" "rare" "mmopt")
SCHEDULES_LEN=${#SCHEDULES_ARR[@]}

if [ $cmd = "run" ] ; then
    if [ $# -lt 1 ] ; then
        error "duration parameter is missing"
    fi
    duration=${1}
    if ! [[ $duration =~ ^[0-9]+$ ]] ; then
        error "given duration '$duration' is not a number"
    fi
    cpus=${2:-1}
    timeout=${3:-$DFLT_AFL_TIMEOUT}
    if [ $cpus = "all" ] ; then
        cpus=$(get_cpus)
        echo "CPU cores available: $cpus"
    fi
    if ! [[ $cpus =~ ^[0-9]+$ ]] ; then
        error "given instances '$cpus' is not a number or 'all'"
    fi
    if ! [[ $timeout =~ ^[0-9]+$ ]] ; then
        error "given timeout '$timeout' is not a number"
    fi
    echo "Running $cpus AFL instances for $duration seconds"
    mkdir -p afl

    # Remove dead screen sessions.
    # Such sessions might remain if the previous run was cancelled.
    screen -wipe || true

    for (( i=0; i<$cpus; i++ )) ; do
        power_schedule=${SCHEDULES_ARR[$(( i % SCHEDULES_LEN))]}
        name=${target}_${i}_${power_schedule}
        id=${i}_${power_schedule}
        if [ $i -eq 0 ] ; then
            # main fuzzer
            fuzz_args="-M $id "
        else
            # secondary fuzzer
            fuzz_args="-S $id "
        fi
        fuzz_args+="-p $power_schedule "
        # TODO: use different fuzzing variants per instance
        fuzz_cmd="./fuzz.sh afl run $target -V $duration $fuzz_args -T $name -t $timeout"
        echo -e "Starting screen session with:\n  $fuzz_cmd"
        screen -dmS afl_$name \
            bash -c "{ $fuzz_cmd >afl/$name.log 2>afl/$name.err ; echo \$? > afl/$name.status; }"
    done
    echo "Started below screen sessions with AFL instances"
    # adding 'true', because screen returns always error, when listing sessions.
    screen -ls afl_ || true

    echo "started=$(date +%s)" > afl/${target}_info
    echo "duration=$duration" >> afl/${target}_info

elif [ $cmd = "watch" ] ; then
    interval=${1:-$DFLT_INTERVAL}
    duration=
    started=$(date +%s)
    # afl/info should include most accurate info on duration and start time
    if [ -f afl/${target}_info ] ; then
        source afl/${target}_info
    fi
    # if no start time given, then get current time (it's better than nothing)
    if [ $started = "none" ] ; then
        stared=$(date +%s)
    fi
    while ! screen -ls afl_${target} | grep "No Sockets found" ; do
        sleep $interval
        # afl folder structure created with some delay after fuzz startup
        if [ -d afl/$target ] ; then
            cargo afl whatsup -d afl/$target
        fi
        now=$(date +%s)
        run_time=$(( now - started ))
        echo "Fuzzing duration : $(humanize_seconds $run_time)"
        if [ $duration != "" ] ; then
            time_left=$(( duration - run_time ))
            if [ $time_left -lt 0 ] ; then
                time_left=0
            fi
            echo "Fuzzing ends in  : $(humanize_seconds $time_left)"
        fi
    done
    echo "AFL sessions stdout:"
    find afl -name "${target}_*.log"  | sort | xargs tail -n50
    echo "AFL sessions stderr:"
    find afl -name "${target}_*.err"  | sort | xargs tail -n50
    list=$(find afl -name "${target}_*.status" | sort)

    echo "AFL sessions info:" | tee afl/sessions_info
    for f in $list ; do
        name=$(basename ${f%.status})
        # fuzzing session id does include target name
        id=${name#${target}_}
        stats_file=afl/${target}/${id}/fuzzer_stats
        stability="n/a"
        coverage="n/a"
        crashes="n/a"
        hangs="n/a"
        execs_cnt="n/a"
        execs_per_sec="n/a"
        if [ -f $stats_file ] ; then
            execs_cnt=$(grep execs_done $stats_file | awk '{print $3}')
            execs_per_sec=$(grep execs_per_sec $stats_file | awk '{print $3}')
            hangs=$(grep saved_hangs $stats_file | awk '{print $3}')
            crashes=$(grep saved_crashes $stats_file | awk '{print $3}')
            stability=$(grep stability $stats_file | awk '{print $3}')
            coverage=$(grep bitmap_cvg $stats_file | awk '{print $3}')
        fi
        d=$(dirname $f)
        printf "  %-30s status:%-2s crashes:%-7s hangs:%-7s execs:%-10s execs/sec:%-7s coverage:%-7s stability:%-7s\n" \
            $name $(cat $f) $crashes $hangs $execs_cnt $execs_per_sec $coverage $stability
    done | tee -a afl/sessions_info
elif [ $cmd = "quit" ] ; then
    list=$(find afl/${target} -name fuzzer_stats | sort)
    if [ "$list" != "" ] ; then
        for stats_file in $list ; do
            name=$(grep afl_banner $stats_file | awk '{print $3}')
            pid=$(grep fuzzer_pid $stats_file | awk '{print $3}')
            echo "killing session $name"
            kill -9 $pid
        done
    else
        echo "nothing to be done"
    fi
else
    error "Command '$cmd' not supported"
fi


