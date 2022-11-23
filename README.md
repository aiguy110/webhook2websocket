# Purpose
Webhooks are a good solution if you need to recieve real-time updates from an server
but if the server is on the public Internet and the application you want to consume
those update from is not, you're kind of out of luck. 

This repo provides a tool to be hosted on a server on the public Internet that listens
for POST request from another server on the public Internet and relays the request 
bodies to subscribed clients on a private network via Websocket connections.