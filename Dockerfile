FROM postgres:15
   
ENV POSTGRES_USER=sqlx
ENV POSTGRES_PASSWORD=sqlx
ENV POSTGRES_DB=sqlx

EXPOSE 5432

USER postgres
CMD ["bash", "-c", "docker-entrypoint.sh postgres"]