WARNING
=======
Under heavy refactoring >:) even master branch...

Status
=======
[![Build Status](https://travis-ci.org/CodersOfTheNight/meowth.svg?branch=master)](https://travis-ci.org/CodersOfTheNight/meowth)

About
=====
Simple service which is receiving messages from zeroMq and pushes them to ElasticSearch. 
A lightweigth Logstash alternative which eats wastly less resources, however has far less functionalitty.  

Compilation
===========
Meowth could either be used with `zMQ` or simple `TCP` consumer.

When using Makefile `ZMQ_CONSUMER=1` flag compiles with `zmq` consumer, by default it builds with `tcp`.

When building manually with `cargo`, same effect is achieved by adding either `--features="zmq"` or `--features="tcp"`,
eg.: `cargo build --features="zmq"`

Be aware, that `zMQ` consumers requires having development libraries on your system


Config
=======
```
---
statsd: # Where metrics about log processing will be sent
  address: 127.0.0.1:8125
  prefix: "logs"
zeromq: # Queue on which metrics will be received
  address: tcp://127.0.0.1:50020
  bind: true # True - says that this app will host queue. Used when no scaling is required
             # False - says that it will connect to some router. Used when there's a need for multiple processors
elastic_search: # Logs will be put here
  address:
    - http://localhost:9200 # It supports multiple ElasticSearch instances
  prefix: logs
  bulk_size: 10 # It will flush each time it collects this amount of logs.
                # Too low number can result in big flood and performance decrease,
                # too high - delay till logs will reach destination and risk to lose higher amount of logs if server crashes
  type: test    # Type by which you will identify these logs in ElasticSearch
```

Usage
=====
(GNU Make is required if you don't have it yet)
`make` - will build
`make release` - will build optimized release version
`make test` - will test
`make install` - will attempt to install service as a daemon (currently only RHEL distros are supported)

Rust compiler should be downloaded automatically

Log protocol
============
Logs are received via ZeroMQ serialized as JSON objects, which has structure as follows:
```
{
  "process": <process_id>,
  "channel": <log record namespace>,
  "func_name": <function name>,
  "args": [<function arguments (Array)>],
  "filename": <file name>,
  "module": <package, namespace, module>,
  "host": <machine host (Optional)>,
  "message": <actual log message>,
  "lineno": <line number>,
  "time": <timestamp>,
  "level": <log level>,
  "type": <log record type>
  
}
```
