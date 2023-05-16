from pyspark.sql import SparkSession

# Create a SparkSession
spark = SparkSession.builder.appName("KafkaConsumer").getOrCreate()

# Define the Kafka topic and server details
topic = "test"
kafka_servers = "localhost:9092"

# Create a DataFrame representing the Kafka messages
df = spark.readStream.format("kafka").option("kafka.bootstrap.servers", kafka_servers).option("subscribe", topic).load()

# Convert the binary key and value columns to strings
df = df.selectExpr("CAST(key AS STRING)", "CAST(value AS STRING)")

# Define a query to print the key and value of each message
query = df.writeStream.outputMode("append").format("console").start()

# Wait for the query to finish
query.awaitTermination()