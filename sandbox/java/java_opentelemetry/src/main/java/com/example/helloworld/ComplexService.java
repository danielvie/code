package com.example.helloworld;

import io.opentelemetry.api.trace.Tracer;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

@Service
public class ComplexService {

    private final Tracer tracer;

    @Autowired
    public ComplexService(Tracer tracer) {
        this.tracer = tracer;
    }

    /**
     * Main operation that orchestrates multiple steps.
     * This method will be traced automatically when called externally (from
     * HelloController).
     */
    public String performComplexOperation(String param) {
        String result = stepOne(param);
        result += stepTwo(param);
        result += stepThree(param);
        return result;
    }

    /**
     * Step One: Simulates database query or external API call
     */
    private String stepOne(String param) {
        try {
            Thread.sleep(100);
            return param+" - Step One Completed. ";
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            return param+" - Step One Failed. ";
        }
    }

    /**
     * Step Two: Simulates business logic processing
     */
    private String stepTwo(String param) {
        try {
            Thread.sleep(200);
            return param+" - Step Two Completed. ";
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            return param+" - Step Two Failed. ";
        }
    }

    /**
     * Step Three: Simulates final processing or response preparation
     */
    private String stepThree(String param) {
        // try {
        // Thread.sleep(300);
        // return param + " - Step Three Completed. ";
        // } catch (InterruptedException e) {
        // Thread.currentThread().interrupt();
        // return param + " - Step Three Failed. ";
        // }
        return TracingUtils.withSpan(tracer, "stepThree", () -> {
            try {
                Thread.sleep(300);
                return param + " - Step Three Completed. ";
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
                return param + " - Step Three Failed. ";
            }
        });
    }
}