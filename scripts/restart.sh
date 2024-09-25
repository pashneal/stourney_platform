# TODO: send shutdown signal to give the server time to save state
pkill -f stourney_server
pkill -f npm

# Start the web client
npm run preview -- --host 0.0.0.0 & 

# Send all logs to file for debugging
RUST_LOG=stourney_server=trace /stourney_platform/server/target/release/stourney_server 2> /persistent/logs &
