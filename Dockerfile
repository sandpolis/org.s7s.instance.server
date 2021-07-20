FROM adoptopenjdk:16-hotspot

# Set application directory
WORKDIR /app

# Set application entry
ENTRYPOINT ["java", "--module-path", "/app/lib", "-m", "com.sandpolis.server.vanilla/com.sandpolis.server.vanilla.Main"]

# Default listening port
EXPOSE 8768

# Set environment
ENV S7S_RUNTIME_RESIDENCY     "container"
ENV S7S_PATH_GEN              "/tmp"
ENV S7S_PATH_LIB              "/app/lib"
ENV S7S_PATH_PLUGIN           "/app/plugin"
ENV S7S_PLUGINS_ENABLED       "true"

# Install application
COPY build/lib /app/lib
