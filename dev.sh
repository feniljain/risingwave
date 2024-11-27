./target/debug/risingwave compactor --listen-addr 0.0.0.0:6660 --advertise-addr localhost:6660 --meta-address http://localhost:5690

./target/debug/risingwave compute --listen-addr 0.0.0.0:5688 --advertise-addr localhost:5688 --meta-address http://localhost:5690

./target/debug/risingwave frontend --listen-addr 0.0.0.0:4566 --meta-addr http://localhost:5690 --advertise-addr localhost:4566

./target/debug/risingwave meta --listen-addr 0.0.0.0:5690 --advertise-addr localhost:5690 --dashboard-host 0.0.0.0:5691 --state-store hummock+s3://kafka-testing-files --data-directory iceberg_hive_local_test --backend mem

./target/debug/risingwave meta --listen-addr 0.0.0.0:5690 --advertise-addr localhost:5690 --dashboard-host 0.0.0.0:5691 --state-store hummock+s3://kafka-testing-files --data-directory iceberg_hive_local_test --backend sql --sql-endpoint postgres://ayoo:ayoo123@localhost:5432/metadata

# 2024-11-20T15:00:06.869891203Z stdout F 20-11-2024 15:00:06,869 DEBUG pool-7-thread-1 QueryId= HeartbeatTask:25 - Sending heartbeat from engine: 10.201.109.0 to cluster manager at: debug-test-2-queue
# 2024-11-20T15:04:55.546150892Z stdout F 20-11-2024 15:04:55,546 DEBUG qtp571514712-227 QueryId= ExecutorHttpService:34 - Received request on engine http server: Path: /, Method: GET

# %d-%d-%dT%d:%d:%d.%dZ {stdout|stderr} F %d-%d-%d %d:%d:%d,%d {DEBUG|ERROR|WARN} %s
# %s-%s-%sT%s:%s:%s.%sZ %s F %s-%s-%s %s:%s:%s,%s %s %s
# format(yr1, mo1, dy1, hr1, mn1, sc1, ms1, fd, dy2, mo2, yr2, hr2, mn2, sc2, ms2, log_type, msg)
