1.First, make sure that java is installed on the machine
     Use java-version to verify whether to install, or download the latest java
     http://www.oracle.com/technetwork/java/javase/downloads/index.html
     Configuration related environment variables.

2. Download and install kafka (kafka_2.11-0.10.2.0 version)
     To the following URL http://kafka.apache.org/downloads.html
3.Turn off kafka message persistence
     Into the kafka directory open /config/server.properties, modify log.retention.hours = 0, the purpose is to turn off kafka message persistence.
4. Start related services (zookeeper, kafka)
     kafka depends on zookeeper, so start zookeeper, and then start kafka, kafka directory
          Start zookeeper: bin / zkServer.sh start

          Start kafka: bin / kafka-server-start.sh config / server.properties
