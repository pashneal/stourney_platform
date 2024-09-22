# Server

## Purpose

This back server is run in the docker environment and collects game information from 
locally played games to be displayed on stourney.com. It collects client information
as the game runs so that the data is able be relayed live as the game progresses

The protocol for how this works is documented below:

## Coming Soon
- [ ] Better documentation with actual shape of json requests 
- [ ] SyncClock updates - allow the server to display how much time is left per player
- [ ] Send game updates when game is actually updated
- [ ] Gameover declaration and ack
- [ ] Sever connection after game end and ack
- [ ] Sever connection on errors


## Protocol 

A client connects via websocket to the server at wss://\<hosted url\>/ws and must
send the following json requests:

check out [`splendor_arena::models`](https://www.github.com/pashneal/splendor_arena)
to see more information

TODO: standardize the format for the json requests

1. Authentication: 

```
# >> Sent from the client
Authentication { api_key : <api_key> } 

# << Received from the server if successful 
Authenticated:Success

# << Recieved from the server if unsuccessful 
Autuhenticated:Failure{ reason : "...<reason>..." }

```

2.  Initialization

```
# >> Sent from the client after authenticated
InitializeGame { info :  <game_info> }

# << Recieved from the server if successful
Initialized::Success{ id : String },

# << Recieved from the server if unsuccessful
Initialized::Failure{ reason: String }
```

3. Updating during the Game

```
# >> Sent from the client as the game progresses
GameUpdates : [ <array of GameUpdate> ]


# TODO: return a set of updates from the server
```

4. Declare game over

```
# >> Sent from the client when the game is over
# must send the number of moves made per game or the server will reject 
GameOver{ total_updates: <num>} 

# TODO: enforce
```
