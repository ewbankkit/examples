import { handleCall, consoleLog, handleAbort } from "wapc-guest-as";
import { Request, Response, ResponseBuilder, Handlers } from "wascc-actor-as/httpserver";
import { Host as KV } from "wascc-actor-as/kv";

export function _start(): void {
  Handlers.handleRequest(handleRequest);
}

function handleRequest(request: Request): Response {
  const bodyAsString = String.UTF8.decode(request.body);

  // Set a key with the body of the request as a string.
    const kv = new KV("");
  const response = kv.set("foo", bodyAsString, 0);
  
  const message = "Response was " + response.value;  
  const payload = String.UTF8.encode(message);
  
  return new ResponseBuilder()
    .withStatusCode(200)
    .withStatus("OK")
    .withBody(payload)
    .build();
}

export function __guest_call(operation_size: usize, payload_size: usize): bool {
  return handleCall(operation_size, payload_size);
}

// Abort function - this should probably be in the actor SDK and not in an actor...
export function abort(
    message: string | null,
    fileName: string | null,
    lineNumber: u32,
    columnNumber: u32
  ): void {
    handleAbort(message, fileName, lineNumber, columnNumber);
  }