package com.demohouse.topup.model.web.response;

import org.springframework.http.HttpStatus;

import java.time.LocalDateTime;

public class ApiResponse<T> {
    protected final LocalDateTime timeStamp;
    protected boolean success;
    protected HttpStatus code;
    protected String message;

    protected T data;

    protected ApiResponse() {
        this.timeStamp = LocalDateTime.now();
    }

    public static <T> ApiResponse<T> success(T data, String message) {
        ApiResponse<T> res = new ApiResponse<>();
        res.success = true;
        res.code = HttpStatus.OK;
        res.message = message;
        res.data = data;
        return res;
    }

    public static <T> ApiResponse<T> success(T data) {
        return success(data, "OK");
    }

    public static <T> ApiResponse<T> failure(HttpStatus code, String message) {
        ApiResponse<T> res = new ApiResponse<>();
        res.success = false;
        res.code = code;
        res.message = message;
        return res;
    }

    public LocalDateTime getTimeStamp() {
        return timeStamp;
    }

    public boolean isSuccess() {
        return success;
    }

    public HttpStatus getCode() {
        return code;
    }

    public String getMessage() {
        return message;
    }

    public T getData() {
        return data;
    }
}

