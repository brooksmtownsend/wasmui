# wasmCloud UI

This actor packages static assets and returns them in response to HTTP requests, effectively packaging user interfaces into a single portable artifact.

This actor makes use of the HTTP server (`wasmcloud:httpserver`) capability in order to accept requests for static assets just as a web server does.

## Configuration

Simply

## Notes

Any static assets above 1MB in size will activate chunking, where those assets are sliced up into approximately 1MB sized chunks and transmitted over wasmCloud RPC. This is required due to the NATS message limit, and while this is not an issue for the majority of cases, it's good to keep in mind that smaller static assets will transfer more efficiently.
