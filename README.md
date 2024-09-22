# stourney_platform
Web platform for https://www.stourney.com

## Docker config

```bash
# builds the docker image
docker build -t stourney_platform .

# creates a docker volume for the database
docker volume create stourney_db

# runs the docker image , exposing the port 4173
# also exposes the api port 3031,  
# and mounts the volume to the container
# the -d flag runs the container in the background
# but -it will run it in the foreground for debugging purposes
docker run -d -p 4173:4173 -p 3031:3031 --mount type=volume,src=stourney_db,target=/persistent stourney_platform

# shows where the docker volume is
docker volume inspect stourney_db

# create a blank sqlite database in the volume  (you'll crash without this)
sqlite3 <path to volume>/stourney.db < server/src/schema.sql

docker ps # to get the container id
docker stop <container id> # to stop the container
```
