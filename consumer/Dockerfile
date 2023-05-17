FROM python:3.11-slim

RUN apt-get update && apt-get install -y default-jdk
ENV JAVA_HOME /usr/lib/jvm/default-jdk

WORKDIR /app

COPY pyspark_consumer.py .

RUN python -m pip install pyspark confluent kafka

ENV PYSPARK_SUBMIT_ARGS="--packages org.apache.spark:spark-sql-kafka-0-10_2.12:3.2.0 pyspark-shell"
CMD ["spark-submit", "pyspark_consumer.py"]