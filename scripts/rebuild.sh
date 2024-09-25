cd /stourney_platform/scripts
./restart.sh

while true; 
do 
  # Check for updates to the stourney platform and rebuild if necessary
  if [[ $(git fetch --dry-run) ]]; then
      echo "Changes detected, rebuilding..."
      git pull
      cd /stourney_platform

      # Alter database if necessary
      sqlite3 /persistent/stourney.db < /stourney_platform/server/src/schema.sql
      cd /stourney_platform/server
      cargo sqlx prepare 

      # Update backend if necessary
      cargo build --release

      # Update frontend if necessary
      cd /stourney_platform/web
      npm install
      npm run build
      
      cd /stourney_platform/scripts
      ./restart.sh
  else
      echo "No changes detected, skipping rebuild..."
  fi
  sleep 5;
done
