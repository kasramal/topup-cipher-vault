package com.demohouse.topup.service.impl;

import com.demohouse.topup.grpc.PinVaultClient;
import com.demohouse.topup.grpc.vault.PinCodeResponse;
import com.demohouse.topup.grpc.vault.ReservationResponse;
import com.demohouse.topup.grpc.vault.StatusResponse;
import com.demohouse.topup.service.PinCodeService;
import org.springframework.stereotype.Service;

import java.io.InputStream;

@Service
public class PinCodeServiceImpl implements PinCodeService {
    private final PinVaultClient pinVaultClient;

    public PinCodeServiceImpl(PinVaultClient pinVaultClient) {
        this.pinVaultClient = pinVaultClient;
    }

    @Override
    public PinCodeResponse getPinCode(String id) {
        return pinVaultClient.getPinCode(id);
    }

    @Override
    public PinCodeResponse takePinCode(String reservationId) {
        return pinVaultClient.takePinCode(reservationId);
    }

    @Override
    public StatusResponse generatePinCode(int count) {
        return pinVaultClient.generatePinCode(count);
    }

    @Override
    public ReservationResponse reservePinCode() {
        return pinVaultClient.reservePinCode();
    }

    @Override
    public StatusResponse uploadPinCodes(InputStream input, String filename) {
        return pinVaultClient.uploadPinCodes(input, filename);
    }
}
