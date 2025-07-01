package com.demohouse.topup.service;

import com.demohouse.topup.grpc.vault.PinCodeResponse;
import com.demohouse.topup.grpc.vault.ReservationResponse;
import com.demohouse.topup.grpc.vault.StatusResponse;

import java.io.InputStream;

public interface PinCodeService {

    PinCodeResponse getPinCode(String id);

    PinCodeResponse takePinCode(String reservationId);

    StatusResponse generatePinCode(int count);

    ReservationResponse reservePinCode();

    StatusResponse uploadPinCodes(InputStream input, String filename);
}