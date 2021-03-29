FROM adoptopenjdk:16-hotspot

# Set application directory
WORKDIR /app

# Set application entry
ENTRYPOINT ["java", "--module-path", "/app/lib", "-m", "com.sandpolis.server.vanilla/com.sandpolis.server.vanilla.Main"]

# Default listening port
EXPOSE 8768

# Set environment
ENV S7S_STORAGE_PROVIDER      "ephemeral"
ENV S7S_NET_CONNECTION_TLS    "true"
ENV S7S_NET_LOGGING_DECODED   "false"
ENV S7S_NET_LOGGING_RAW       "false"
ENV S7S_PATH_GEN              "/tmp"
ENV S7S_PATH_LIB              "/app/lib"
ENV S7S_PATH_PLUGIN           "/app/plugin"
ENV S7S_PLUGINS_ENABLED       "true"

# Enable JVM debugging
#ENV JAVA_TOOL_OPTIONS "-agentlib:jdwp=transport=dt_socket,address=0.0.0.0:7000,server=y,suspend=y"

# Install application
COPY build/lib /app/lib
