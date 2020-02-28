initSidebarItems({"enum":[["WriteMode","Indicate whether a write method should leave the current gRPC method invocation open or close it."]],"fn":[["build_status","Helper to create a gRPC status object."],["decap_response","Extract a protocol buffer message from a GrpcResponse wrapper. Returns the message together with an indicator of whether this is the last response."],["encap_request","Encapsulate a protocol buffer message in a GrpcRequest wrapper using the given method name."],["handle_req_rsp","Generic function that handles request deserialization and response serialization (and sending down a channel) for a function that handles a request/response pair."],["handle_req_stream","Generic function that handles request deserialization and response serialization (and sending down a channel) for a function that handles a request and streams responses."],["handle_stream_rsp","Generic function that handles request deserialization and response serialization (and sending down a channel) for a function that handles a stream of requests to produce a response."],["handle_stream_stream","Generic function that handles request deserialization and response serialization (and sending down a channel) for a function that handles a stream of requests to produce a stream of responses."],["invoke_grpc_method","Helper to inject a (single) gRPC request message via a notification channel, in the same manner as the gRPC pseudo-Node does, and collect a (single) response."],["invoke_grpc_method_stream","Helper to inject a (single) gRPC request message via a notification channel, in the same manner as the gRPC pseudo-Node does, and return a channel for reading responses from."]],"mod":[["client",""]],"struct":[["ChannelResponseWriter","Channel-holding object that encapsulates response messages into `GrpcResponse` wrapper messages and writes serialized versions to a send channel."],["Invocation","A gRPC invocation, consisting of exactly two channels: one to read incoming requests from the client, and one to write outgoing responses to the client."]],"trait":[["ServerNode","Trait for Oak Nodes that act as a gRPC services."]],"type":[["Result","Result type that uses a [`proto::status::Status`] type for error values."]]});