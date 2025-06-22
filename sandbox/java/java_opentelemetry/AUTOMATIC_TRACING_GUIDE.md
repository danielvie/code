# OpenTelemetry Java Agent Auto-Instrumentation Setup

The OpenTelemetry Java Agent provides automatic instrumentation for Spring Boot applications without any code changes.

## Setup Instructions:

### 1. Download the OpenTelemetry Java Agent
```bash
# Download the latest agent
curl -L -o opentelemetry-javaagent.jar https://github.com/open-telemetry/opentelemetry-java-instrumentation/releases/latest/download/opentelemetry-javaagent.jar
```

### 2. Run your application with the agent
```bash
# Windows (PowerShell)
java -javaagent:opentelemetry-javaagent.jar `
     -Dotel.service.name=helloworld-service `
     -Dotel.exporter.otlp.endpoint=http://localhost:4317 `
     -jar build/libs/demo-0.0.1-SNAPSHOT.jar

# Alternative: Set environment variables
$env:OTEL_SERVICE_NAME="helloworld-service"
$env:OTEL_EXPORTER_OTLP_ENDPOINT="http://localhost:4317"
java -javaagent:opentelemetry-javaagent.jar -jar build/libs/demo-0.0.1-SNAPSHOT.jar
```

### 3. What gets automatically instrumented:
- All HTTP requests (Spring Web)
- Database calls (JDBC, JPA, Hibernate)
- External HTTP calls (RestTemplate, WebClient)
- JMS, RabbitMQ, Kafka messaging
- And much more...

### 4. Benefits:
- ✅ Zero code changes required
- ✅ Industry-standard instrumentation
- ✅ Automatic correlation across services
- ✅ Rich metadata and attributes
- ✅ Performance optimized

### 5. Combined approach:
You can use the Java Agent together with your custom TracingAspect for:
- Automatic instrumentation of frameworks
- Custom business logic tracing with your aspect

### 6. Configuration options:
Add these to application.properties or as system properties:

```properties
# Service identification
otel.service.name=helloworld-service
otel.service.version=1.0.0

# Exporter configuration
otel.exporter.otlp.endpoint=http://localhost:4317
otel.exporter.otlp.protocol=grpc

# Resource attributes
otel.resource.attributes=service.namespace=dev,deployment.environment=local

# Sampling (1.0 = 100%, 0.1 = 10%)
otel.traces.sampler=traceidratio
otel.traces.sampler.arg=1.0

# Disable specific instrumentations if needed
otel.instrumentation.spring-web.enabled=true
otel.instrumentation.jdbc.enabled=true
otel.instrumentation.jms.enabled=false
```
