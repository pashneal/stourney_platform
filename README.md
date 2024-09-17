# stourney_platform
Web platform for https://www.stourney.com

## Docker config

```bash
# builds the docker image
docker build -t stourney_platform .

# creates a docker volume for the database
docker volume create stourney_db

# runs the docker image , exposing the port 4173 and mounting the volume
docker run -dp 4173:4173 --mount type=volume,src=stourney_db,target=/persistent stourney_platform

# shows where the docker volume is
docker volume inspect stourney_db
```
