FROM rust:latest
#This line tells Docker to use the latest official Rust image as the base image for our new image. This image includes the Rust compiler and other dependencies needed to build and run Rust applications.

WORKDIR /usr/src/app
# This line sets the working directory in the container to /usr/src/app. This means that any subsequent commands will be run from this directory.

COPY . .
# This line copies the current directory (i.e., the directory containing the Dockerfile) into the container at the current working directory (/usr/src/app). This makes our application code available inside the container.

RUN cargo build --release
# This line runs the cargo build command with the --release flag, which builds our Rust application in release mode (i.e., with optimizations enabled). This command is run inside the container.

CMD cargo run --quiet
# This line sets the default command to run when the container is started. In this case, it runs our Rust application using the cargo run command.

# to create image in docker: docker build . -t repetitionindocker:v1
# to run image in docker: docker run -d -p [port you want to listen to on the host f.ex 8080]:[port you expose on the server f.ex 3000] repetitionindocker:v1
# to see the logs of the server in the docker: docker logs [new container id]