# Kafka IoT Data Processing Simulation (WIP)
*This repository is for simulating the dynamic data flow of an IoT environment right on your local PC. It can be used for understanding the system's various technologies and further to derive implementaion concepts.*

## Quickstart
### Prerequisites
* [WSL 2](https://learn.microsoft.com/en-us/windows/wsl/install) (Windows)
* [Docker Desktop](https://www.docker.com/get-started/) with Docker Compose V2 activated

### Starting Kafka IoT Data Processing Simulation
The easiest way to start this simulation is by running the following command and exploring the different components in Docker Desktop:
```Bash
git clone https://github.com/NotAndex/kafka_iot_sim.git
cd kafka_iot_sim
docker compose up -d
```
To stop the simulation and remove all containers and volumes, use the following command:
```Bash
docker compose down --volume 
```

### Changing the Behavior of the Simulation

The metadata file responsible for the instantiation of the simulation is the [`docker-compose.yml`](docker-compose.yml) file. Each component described in the Architecture section (see below) is represented as a service within this file. By modifying this file, various aspects of the simulation can be altered, including:

* The number of IoT devices by adding more `iot_data_gen` services
* The structure of the IoT devices, such as:
  * Sensor quantity
  * Sensor names
  * Observation frequency  
  ```yml
  # By changing this parameter of the iot_data_gen service in the docker-compose.yml
  [{"name":"sensor_1","properties":{"obs_frequency":"1"}},
   {"name":"sensor_2","properties":{"obs_frequency":"1"}}]
  ```
* The number of topics by adding more `create_topic` services
* The quantity and type of consumers by adding more `consumer` services
* The number of brokers

It is highly recommended to start exploring and experimenting with different configurations of the `docker-compose.yml` file. In the worst-case scenario, you can always revert back to the initial setup. Additionally, please note that more properties of the services can be found within their dedicated folders `docker-compose.yml`, which are hidden at the root level of this repository. Don't hesitate to explore those as well.

### Further Local Development
...

## Architecture Overview
TODO: add IMG

### Producer (iot_data_gen)
A *producer* is a client application that pushes (writes) events to a Kafka *broker*. In this simulation, the client application is an IoT device that comprises multiple sensors. The observations (events) are written to a *topic*, where each sensor represents a key in the *topic*.

### Topic (create_topic)
As already mentioned, *events* (observations) are stored in *topics*. In general, one topic can have multiple subscribed *producers* and *consumers*. However, in this use-case, there is a one-to-one relation between a topic and an IoT device.

For scalability, one topic can be partitioned and spread across multiple brokers. For fault tolerance and high availability, it can be replicated.

### Kafka Cluster (kafka_cluster)
Kafka is run as a cluster of one or more servers. One type of server in the storage layer is called a *broker*, while the other type is [Kafka Connect](https://kafka.apache.org/documentation/#connect). Kafka Connect is an out-of-the-box tool for data integration with existing systems. Although Kafka Connect is not used in this simulation, it is worth further investigation of its capabilities.

#### Broker
A Kafka cluster usually comprises several *brokers* that ensure load balancing. These brokers are designed to be stateless and rely on *ZooKeeper* for managing their cluster's state. A single Kafka *broker* instance has the capability to handle a substantial number of read and write operations per second. Moreover, each *broker* can efficiently manage terabytes of messages without experiencing any performance degradation. The process of electing a leader for a Kafka *broker* can be facilitated through *ZooKeeper*.

### Consumer (consumer)
...

## Credits
* [CONFLUENT](https://www.confluent.io/) for providing such a nice Kafka Docker Image and documentation.
* [https://kafka.apache.org/documentation/](https://kafka.apache.org/documentation/) for being a valuable source of information.


