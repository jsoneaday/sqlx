FROM postgres:15

RUN apt-get update && \
    apt-get install -y build-essential \
    pkg-config \
    libssl-dev \
    curl \
    jq \
    && apt-get clean
    
ENV POSTGRES_USER=sqlx
ENV POSTGRES_PASSWORD=sqlx
ENV POSTGRES_DB=sqlx

EXPOSE 5432

COPY . /usr/local/bin/db
WORKDIR /usr/local/bin/db

USER postgres
CMD ["bash", "-c", "docker-entrypoint.sh postgres"]