# TODO: send shutdown signal to give the server time to save state
pkill -f stourney_server
pkill -f npm
pkill -f node
pkill -f vite

# Start the web client
cd /stourney_platform/web
npm run preview -- --host 0.0.0.0 2> /persistent/logs & 

# Send all logs to file for debugging
cd /stourney_platform/server
RUST_LOG=stourney_server=trace /stourney_platform/server/target/release/stourney_server 2> /persistent/logs &
