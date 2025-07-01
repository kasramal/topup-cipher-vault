package com.demohouse.topup.grpc;

import com.demohouse.topup.grpc.vault.*;
import com.google.protobuf.ByteString;
import com.google.protobuf.Empty;
import io.grpc.stub.StreamObserver;
import net.devh.boot.grpc.client.inject.GrpcClient;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Component;

import java.io.InputStream;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;

@Component
public class PinVaultClient {
    @GrpcClient("rust-pin-service")
    private PinCodeVaultServiceGrpc.PinCodeVaultServiceBlockingStub blockingStub;

    @GrpcClient("rust-pin-service")
    private PinCodeVaultServiceGrpc.PinCodeVaultServiceStub asyncStub;

    private static final Logger LOGGER = LoggerFactory.getLogger(PinVaultClient.class);

    public PinCodeResponse getPinCode(String id) {
        IdRequest request = IdRequest.newBuilder()
                .setId(id).build();
        return blockingStub.getPinCode(request);
    }

    public PinCodeResponse takePinCode(String id) {
        IdRequest request = IdRequest.newBuilder()
                .setId(id).build();
        return blockingStub.takePinCode(request);
    }

    public StatusResponse generatePinCode(int count) {
        GenerationRequest request = GenerationRequest.newBuilder()
                .setCount(count).build();
        return blockingStub.generatePinCode(request);
    }

    public ReservationResponse reservePinCode() {
        return blockingStub.reservePinCode(Empty.getDefaultInstance());
    }

    public StatusResponse uploadPinCodes(InputStream input, String fileName) {
        LOGGER.info("Starting upload of file: {}", fileName);

        CompletableFuture<StatusResponse> responseFuture = new CompletableFuture<>();

        StreamObserver<StatusResponse> responseObserver = new StreamObserver<>() {
            @Override
            public void onNext(StatusResponse value) {
                LOGGER.info("Upload response: {}", value.getMessage());
                responseFuture.complete(value); // set response
            }

            @Override
            public void onError(Throwable t) {
                LOGGER.error("Upload failed: {}", t.getMessage(), t);
                responseFuture.completeExceptionally(t); // set error
            }

            @Override
            public void onCompleted() {
                LOGGER.info("Upload stream completed.");
            }
        };

        StreamObserver<PinCodeChunk> requestObserver = asyncStub.uploadPinCodes(responseObserver);

        try {
            byte[] buffer = new byte[1024];
            int bytesRead;

            if ((bytesRead = input.read(buffer)) != -1) {
                requestObserver.onNext(PinCodeChunk.newBuilder()
                        .setFileName(fileName)
                        .setContent(ByteString.copyFrom(buffer, 0, bytesRead))
                        .build());
            }

            while ((bytesRead = input.read(buffer)) != -1) {
                requestObserver.onNext(PinCodeChunk.newBuilder()
                        .setContent(ByteString.copyFrom(buffer, 0, bytesRead))
                        .build());
            }

            requestObserver.onCompleted();
        } catch (Exception e) {
            requestObserver.onError(e);
            responseFuture.completeExceptionally(e);
        }

        try {
            return responseFuture.get(); // Blocks until response or error
        } catch (InterruptedException | ExecutionException e) {
            throw new RuntimeException("Upload failed", e);
        }
    }
}
