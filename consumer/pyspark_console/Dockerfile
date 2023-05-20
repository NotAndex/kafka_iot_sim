FROM python:3.11-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    openjdk-11-jdk \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY main.py .

RUN python -m pip install pyspark

CMD ["spark-submit", "--packages", "org.apache.spark:spark-sql-kafka-0-10_2.12:3.4.0", "main.py"]