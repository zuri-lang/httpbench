#!/bin/bash
here=$(cd "$(dirname "$0")" && pwd)
port=${1:-9000}
exec "$here/../../sqlbench/ruby/bin/falcon" serve --bind "http://127.0.0.1:$port" --config "$here/config.ru" --count 1

