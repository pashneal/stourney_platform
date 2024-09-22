FROM nealpowell/stourney-base:latest 
#RUN apt-get update 
#RUN apt-get install -y build-essential
#RUN apt-get install -y curl
#RUN apt-get install -y git
#RUN apt-get install -y libssl-dev
#RUN apt-get install -y pkg-config

#RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
#ENV PATH="/root/.cargo/bin:${PATH}"
#RUN apt-get install npm -y

RUN git clone https://github.com/pashneal/stourney_platform 
RUN cat stourney_platform/server/Cargo.toml | grep arena

RUN cd stourney_platform/server
RUN apt-get install -y sqlite3 libsqlite3-dev 
RUN mkdir /persistent
WORKDIR /stourney_platform/server
ENV DATABASE_URL="sqlite:///persistent/stourney.db"
RUN sqlite3 /persistent/stourney.db < src/schema.sql
RUN cargo install sqlx-cli
RUN cargo sqlx prepare
RUN cargo build --release 

RUN cd /stourney_platform/web 
WORKDIR /stourney_platform/web
RUN npm install
RUN npm run build

RUN echo "npm run preview -- --host 0.0.0.0 &" >> start.sh
RUN echo "RUST_LOG=info /stourney_platform/server/target/release/stourney_server" >> start.sh
RUN cat ./start.sh
RUN chmod +x ./start.sh
 
# Note that this is not a production ready setup.
# but it is good enough for a simple demo
CMD ["bash", "./start.sh"]
# Web server
EXPOSE 4173 
# Database api
EXPOSE 3031
