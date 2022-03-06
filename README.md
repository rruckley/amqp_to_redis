# amqp_to_redis
Simple micro-service to  take payloads from RabbitMQ queues and cache them in REDIS. REDIS key is the message id so make sure you set this correctly when queueing messages into RabbitMQ.
