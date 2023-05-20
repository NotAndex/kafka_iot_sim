import os

from pyspark.sql import SparkSession
from pyspark.sql.functions import (
    col,
    date_format,
    from_json,
    from_utc_timestamp,
)
from pyspark.sql.types import StringType, StructField, StructType, TimestampType

# Create a SparkSession
spark = SparkSession.builder \
    .appName("KafkaConsumer") \
    .config("spark.sql.extensions", "io.delta.sql.DeltaSparkSessionExtension") \
    .config("spark.sql.catalog.spark_catalog", "org.apache.spark.sql.delta.catalog.DeltaCatalog") \
    .getOrCreate()

bootstrap_server = os.environ.get('BOOTSTRAP_SERVER')
topic_name = os.environ.get("TOPIC")

# Create a DataFrame representing the Kafka messages
df = spark.readStream.format("kafka").option("kafka.bootstrap.servers", bootstrap_server).option("subscribe", topic_name) .load()

# Convert the binary key and value columns to strings
df = df.selectExpr("CAST(key AS STRING)", "CAST(value AS STRING)")

# Define the schema for the Delta table
schema = StructType([
    StructField("timestamp", TimestampType(), nullable=False),
    StructField("value", StringType(), nullable=False)
])

# Parse the JSON value column and extract the timestamp and value fields
df = df.withColumn("jsonData", from_json(col("value"), schema)) \
    .selectExpr("key AS sensor_name", "jsonData.*")  # Rename the key column to "sensor_name"

# Perform truncation and UTC conversion inplace on the index column
df = df.withColumn("timestamp", from_utc_timestamp(col("timestamp").cast("timestamp"), "UTC"))

# Extract the year, month, day, hour, and minute from the timestamp
df = df.withColumn("year", date_format(col("timestamp"), "yyyy"))
df = df.withColumn("month", date_format(col("timestamp"), "MM"))
df = df.withColumn("day", date_format(col("timestamp"), "dd"))
df = df.withColumn("hour", date_format(col("timestamp"), "HH"))
df = df.withColumn("minute", date_format(col("timestamp"), "mm"))

# Define the partition columns
partition_columns = ["year", "month", "day", "hour", "minute"]

# Start the streaming query and specify the process_batch function as the foreachBatch operation
query = df.writeStream.format("delta") \
    .outputMode("append") \
    .partitionBy(partition_columns) \
    .option("checkpointLocation", "/app/delta_lake/bronze_checkpoint") \
    .start("/app/delta_lake/bronze")
# Wait for the streaming query to finish
query.awaitTermination()