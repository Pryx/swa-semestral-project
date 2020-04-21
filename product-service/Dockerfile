FROM openjdk:11
VOLUME /tmp

ADD https://github.com/vishnubob/wait-for-it/blob/master/wait-for-it.sh wait-for-it.sh
RUN chmod +x wait-for-it.sh

ARG JAR_FILE=target/*.jar
COPY ${JAR_FILE} app.jar
EXPOSE 8081

# ENTRYPOINT ["./wait-for-it.sh", "-t 60", "mongo:27017", "--", "java","-Djava.security.egd=file:/dev/./urandom","-XX:+UseContainerSupport", "-jar", "app.jar"]
ENTRYPOINT ["java","-Djava.security.egd=file:/dev/./urandom","-XX:+UseContainerSupport","-jar","/app.jar"]
