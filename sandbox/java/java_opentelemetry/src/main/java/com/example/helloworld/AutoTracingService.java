package com.example.helloworld;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

/**
 * Example service to demonstrate automatic tracing.
 * 
 * With the enhanced TracingAspect, all public methods in this service
 * will automatically be traced without any annotations needed.
 */
@Service
public class AutoTracingService {
    
    @Autowired
    private ComplexService complexService;
    
    /**
     * This method will be automatically traced as "ExampleAutoTracedService.processData"
     */
    public String processData(String input) {
        // This will create a span automatically
        String processed = "Processed: " + input;
          // This call will also be traced since ComplexService methods will be auto-traced
        String complexResult = complexService.performComplexOperation("auto-trace-demo");
        
        return processed + " | " + complexResult;
    }
    
    /**
     * This method will also be automatically traced
     */
    public int calculateValue(int a, int b) {
        // Simulate some work
        try {
            Thread.sleep(50);
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
        
        return a * b + 10;
    }
    
    /**
     * This method demonstrates error handling in automatic tracing
     */
    public String riskyOperation(boolean shouldFail) {
        if (shouldFail) {
            throw new RuntimeException("Simulated error for tracing demo");
        }
        return "Success!";
    }
    
    // This method will NOT be traced because it's a getter
    public String getServiceName() {
        return "ExampleAutoTracedService";
    }
    
    // This method will NOT be traced because it's a setter
    public void setData(String data) {
        // setter logic
    }
}
