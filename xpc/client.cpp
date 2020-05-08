#include <benchmark/benchmark.h>
#include <stdio.h>
#include <stdlib.h>
#include <xpc/xpc.h>

static xpc_connection_t connect()
{
  xpc_connection_t conn = xpc_connection_create_mach_service("com.example.benchmark", NULL, 0);
  if (conn == NULL) {
    perror("xpc_connection_create_mach_service");
    return nullptr;
  }

  xpc_connection_set_event_handler(conn, ^(xpc_object_t obj) {
    printf("Received message in generic event handler: %p\n", obj);
    printf("%s\n", xpc_copy_description(obj));
  });

  xpc_connection_resume(conn);

  return conn;
}

static void write_and_read(xpc_connection_t conn, std::vector<uint8_t>& value)
{
  xpc_object_t msg = xpc_dictionary_create(NULL, NULL, 0);
  xpc_dictionary_set_data(msg, "k", value.data(), value.size());

  xpc_connection_send_message_with_reply(conn, msg, NULL, ^(xpc_object_t resp) {
    //printf("Received reply: %p\n", resp);
    //printf("%s\n", xpc_copy_description(resp));
  });
}

static void write_and_read_10b(benchmark::State& state)
{
  auto conn = connect();
  std::vector<uint8_t> value(10, 1);

  for (auto _ : state) {
    write_and_read(conn, value);
  }
}

static void write_and_read_1kib(benchmark::State& state)
{
  auto conn = connect();
  std::vector<uint8_t> value(1024, 1);

  for (auto _ : state) {
    write_and_read(conn, value);
  }
}


static void write_and_read_10mib(benchmark::State& state)
{
  auto conn = connect();
  std::vector<uint8_t> value(10 * 1024 * 1024, 1);

  for (auto _ : state) {
    write_and_read(conn, value);
  }
}

static void connect_write_and_read_10b(benchmark::State& state)
{
  std::vector<uint8_t> value(10, 1);
  for (auto _ : state) {
    write_and_read(connect(), value);
  }
}

BENCHMARK(write_and_read_10b);
BENCHMARK(write_and_read_1kib);
BENCHMARK(write_and_read_10mib);
BENCHMARK(connect_write_and_read_10b);

BENCHMARK_MAIN();
