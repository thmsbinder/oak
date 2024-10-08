//
// Copyright 2023 The Project Oak Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

package com.google.oak.transport;

import com.google.oak.crypto.v1.EncryptedRequest;
import com.google.oak.crypto.v1.EncryptedResponse;
import com.google.oak.session.v1.EndorsedEvidence;
import com.google.oak.session.v1.GetEndorsedEvidenceRequest;
import com.google.oak.session.v1.GetEndorsedEvidenceResponse;
import com.google.oak.session.v1.InvokeRequest;
import com.google.oak.session.v1.RequestWrapper;
import com.google.oak.session.v1.ResponseWrapper;
import com.google.oak.util.Result;
import io.grpc.stub.StreamObserver;
import java.time.Duration;
import java.util.function.Function;
import java.util.logging.Level;
import java.util.logging.Logger;

public class GrpcStreamingTransport implements EvidenceProvider, Transport {
  private static final Logger logger = Logger.getLogger(GrpcStreamingTransport.class.getName());

  public static final Duration DEFAULT_TIMEOUT = Duration.ofSeconds(10);

  private final Duration timeout;

  /**
   * QueueingStreamObserver with a queue containing responses received from the
   * server.
   * The queue size is 1 because we expect to receive a single response message
   * for each request.
   */
  QueueingStreamObserver<ResponseWrapper> responseObserver = new QueueingStreamObserver<>(1);
  private final StreamObserver<RequestWrapper> requestObserver;

  /**
   * Creates an instance of {@code GrpcStreamingTransport}.
   *
   * The response timeout will be the value of {@code DEFAULT_TIMEOUT}.
   *
   * @param stream a method reference to a gRPC client streaming method with the
   *               appropriate request
   *               and response types.
   */
  public GrpcStreamingTransport(
      Function<StreamObserver<ResponseWrapper>, StreamObserver<RequestWrapper>> stream) {
    this(stream, DEFAULT_TIMEOUT);
  }

  /**
   * Creates an instance of {@code GrpcStreamingTransport}.
   *
   * @param stream a method reference to a gRPC client streaming method with the
   *               appropriate request
   *               and response types.
   * @param timeout The length of time to wait for a {@code InvokeResponse}
   *                after sending an @{code InvokeRequest}.
   */
  public GrpcStreamingTransport(
      Function<StreamObserver<ResponseWrapper>, StreamObserver<RequestWrapper>> stream,
      Duration timeout) {
    this.requestObserver = stream.apply(responseObserver);
    this.timeout = timeout;
  }

  /**
   * Returns evidence about the trustworthiness of a remote server.
   *
   * @return {@code EndorsedEvidence} wrapped in a {@code Result}
   */
  @Override
  public Result<EndorsedEvidence, String> getEndorsedEvidence() {
    RequestWrapper requestWrapper =
        RequestWrapper.newBuilder()
            .setGetEndorsedEvidenceRequest(GetEndorsedEvidenceRequest.newBuilder())
            .build();
    logger.log(Level.INFO, "sending get endorsed evidence request: " + requestWrapper);

    return receiveResponse(requestWrapper).map(responseWrapper -> {
      logger.log(Level.INFO, "received get endorsed evidence response: " + responseWrapper);
      return responseWrapper.getGetEndorsedEvidenceResponse().getEndorsedEvidence();
    });
  }

  /**
   * Sends a request to the enclave and returns a response.
   *
   * @param encryptedRequest {@code oak.crypto.EncryptedRequest} proto message
   * @return {@code oak.crypto.EncryptedResponse} proto message wrapped in a {@code Result}
   */
  @Override
  public Result<EncryptedResponse, String> invoke(EncryptedRequest encryptedRequest) {
    RequestWrapper requestWrapper =
        RequestWrapper.newBuilder()
            .setInvokeRequest(InvokeRequest.newBuilder().setEncryptedRequest(encryptedRequest))
            .build();
    logger.log(Level.INFO, "sending invoke request: " + requestWrapper);

    return receiveResponse(requestWrapper).map(responseWrapper -> {
      logger.log(Level.INFO, "received invoke response: " + responseWrapper);
      return responseWrapper.getInvokeResponse().getEncryptedResponse();
    });
  }

  @Override
  public void close() throws Exception {
    this.requestObserver.onCompleted();
  }

  private Result<ResponseWrapper, String> receiveResponse(RequestWrapper requestWrapper) {
    this.requestObserver.onNext(requestWrapper);
    try {
      // TODO(#3644): Add retry for client messages.
      ResponseWrapper responseWrapper = this.responseObserver.poll(this.timeout);
      if (responseWrapper == null) {
        return Result.error("No response message received within the specified timeout.");
      } else {
        return Result.success(responseWrapper);
      }
    } catch (InterruptedException e) {
      Thread.currentThread().interrupt();
      return Result.error("Thread interrupted while waiting for a response");
    }
  }
}
