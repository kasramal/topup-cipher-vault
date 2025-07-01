package com.demohouse.topup.controller;


import com.demohouse.topup.grpc.vault.PinCodeResponse;
import com.demohouse.topup.grpc.vault.ReservationResponse;
import com.demohouse.topup.grpc.vault.StatusResponse;
import com.demohouse.topup.model.web.response.ApiResponse;
import com.demohouse.topup.model.web.response.request.GenerationReqDto;
import com.demohouse.topup.model.web.response.request.TakePinCodeReqDto;
import com.demohouse.topup.service.PinCodeService;
import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;

import java.io.IOException;

@RestController
@RequestMapping("/api/v1/pin-code")
public class PinCodeController {

    private final PinCodeService pinCodeService;

    public PinCodeController(PinCodeService pinCodeService) {
        this.pinCodeService = pinCodeService;
    }

    @GetMapping("/test")
    public ApiResponse<?> test() {
        return ApiResponse.success(1123213);
    }

    @GetMapping("/{id}")
    public ApiResponse<?> get(@PathVariable("id") String id) {
        PinCodeResponse response = pinCodeService.getPinCode(id);
        if (response.getSuccess())
            return ApiResponse.success(response.getPinCode());
        else
            return ApiResponse.failure(
                    HttpStatus.INTERNAL_SERVER_ERROR,
                    response.getMessage()
            );
    }

    @PostMapping("/generate")
    public ApiResponse<?> generate(@RequestBody GenerationReqDto req) {
        StatusResponse response = pinCodeService.generatePinCode(req.getCount());
        if (response.getSuccess())
            return ApiResponse.success(response.getMessage());
        else
            return ApiResponse.failure(
                    HttpStatus.INTERNAL_SERVER_ERROR,
                    response.getMessage()
            );
    }

    @PostMapping("/reserve")
    public ApiResponse<?> reserve() {
        ReservationResponse response = pinCodeService.reservePinCode();
        if (response.getSuccess())
            return ApiResponse.success(response.getId());
        else
            return ApiResponse.failure(
                    HttpStatus.INTERNAL_SERVER_ERROR,
                    response.getMessage()
            );
    }

    @PostMapping("/take")
    public ApiResponse<?> take(@RequestBody TakePinCodeReqDto dto) {
        PinCodeResponse response = pinCodeService.takePinCode(dto.getReservationId());
        if (response.getSuccess())
            return ApiResponse.success(response.getPinCode());
        else
            return ApiResponse.failure(
                    HttpStatus.INTERNAL_SERVER_ERROR,
                    response.getMessage()
            );
    }

    @PostMapping("/upload")
    public ApiResponse<String> uploadFile(@RequestParam("file") MultipartFile file) throws IOException {
        StatusResponse response = pinCodeService.uploadPinCodes(file.getInputStream(), file.getName());
        if (response.getSuccess())
            return ApiResponse.success(response.getMessage());
        else
            return ApiResponse.failure(
                    HttpStatus.INTERNAL_SERVER_ERROR,
                    response.getMessage()
            );
    }
}

