package com.example.helloworld;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class HelloController {
    @Autowired
    private ComplexService complexService;
    
    @Autowired
    private AutoTracingService autoTracingService;

    @GetMapping("/")
    public String sayHello() {
        return complexService.performComplexOperation("");
    }

    @GetMapping("/{param}")
    public String sayHello(@PathVariable String param) {
        System.err.println("My param is " + param);
        return complexService.performComplexOperation(param);
    }
    
    /**
     * This will create a trace with multiple spans automatically:
     * 1. HelloController.testAutoTracing (this method)
     * 2. ExampleAutoTracedService.processData
     * 3. ComplexService.performComplexOperation (called from processData)
     * 4. All the internal methods in ComplexService
     */
    @GetMapping("/auto-trace/{input}")
    public String testAutoTracing(@PathVariable String input) {
        // This will be automatically traced
        String result = autoTracingService.processData(input);
        
        // This will also be automatically traced
        int calculation = autoTracingService.calculateValue(5, 10);
        
        return result + " | Calculation: " + calculation;
    }
    
    /**
     * Endpoint to test error tracing
     */
    @GetMapping("/auto-trace/error/{shouldFail}")
    public String testErrorTracing(@PathVariable boolean shouldFail) {
        try {
            return autoTracingService.riskyOperation(shouldFail);
        } catch (RuntimeException e) {
            return "Error caught: " + e.getMessage();
        }
    }
}
