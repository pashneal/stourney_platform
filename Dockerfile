FROM nealpowell/stourney-cache:latest 

RUN apt-get install -y procps

RUN git pull

RUN cd /stourney_platform/server
WORKDIR /stourney_platform/server
ENV DATABASE_URL="sqlite:///persistent/stourney.db"

# Alter the schema to include any updates
RUN sqlite3 /persistent/stourney.db < src/schema.sql
RUN cargo sqlx prepare
# Rebuild the backend binaries
RUN cargo build --release 

# Rebuild the web client
RUN cd /stourney_platform/web 
WORKDIR /stourney_platform/web
RUN npm install
RUN npm run build

RUN chmod +x /stourney_platform/scripts/restart.sh
RUN chmod +x /stourney_platform/scripts/rebuild.sh
 
# Note that this is not a production ready setup.
# but it is good enough for a simple demo
CMD ["bash", "/stourney_platform/scripts/rebuild.sh"]
# Web server
EXPOSE 4173 
# Database api
EXPOSE 3031
