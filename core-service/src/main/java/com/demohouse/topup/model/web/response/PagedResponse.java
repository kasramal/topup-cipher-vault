package com.demohouse.topup.model.web.response;

import org.springframework.data.domain.Page;
import org.springframework.http.HttpStatus;

import java.util.List;

public class PagedResponse<T> extends ApiResponse<List<T>> {
    private long totalElements;
    private int totalPages;
    private int page;
    private int size;

    private PagedResponse() {
        super();
    }

    public static <T> PagedResponse<T> fromPage(Page<T> pageData) {
        PagedResponse<T> res = new PagedResponse<>();
        res.success = true;
        res.code = HttpStatus.OK;
        res.message = "OK";
        res.data = pageData.getContent();
        res.totalElements = pageData.getTotalElements();
        res.totalPages = pageData.getTotalPages();
        res.page = pageData.getNumber();
        res.size = pageData.getSize();
        return res;
    }

    public long getTotalElements() {
        return totalElements;
    }

    public int getTotalPages() {
        return totalPages;
    }

    public int getPage() {
        return page;
    }

    public int getSize() {
        return size;
    }
}

