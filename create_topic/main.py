import os

from confluent_kafka.admin import AdminClient, NewTopic

bootstrap_server = os.environ.get('BOOTSTRAP_SERVER')
topic_name = os.environ.get("TOPIC")


admin_client = AdminClient({"bootstrap.servers": bootstrap_server})
num_partitions = 1
replication_factor = 1

new_topic = NewTopic(topic_name, num_partitions, replication_factor)

# Create the topic
topic_result = admin_client.create_topics([new_topic])

# Check if the topic was successfully created
for topic, result in topic_result.items():
    try:
        result.result()  # Raises an exception if an error occurred
        print(f"Topic '{topic}' has been created")
    except Exception as e:
        print(f"Failed to create topic '{topic}': {e}")