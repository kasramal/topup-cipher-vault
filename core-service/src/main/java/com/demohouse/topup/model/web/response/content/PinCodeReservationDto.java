package com.demohouse.topup.model.web.response.content;

import java.util.Date;

public class PinCodeReservationDto {

    private String reservationId;
    private Date expirationDate;

    public String getReservationId() {
        return reservationId;
    }

    public void setReservationId(String reservationId) {
        this.reservationId = reservationId;
    }

    public Date getExpirationDate() {
        return expirationDate;
    }

    public void setExpirationDate(Date expirationDate) {
        this.expirationDate = expirationDate;
    }
}

