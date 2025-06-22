package com.example.helloworld;

import io.opentelemetry.api.trace.Span;
import io.opentelemetry.api.trace.Tracer;
import io.opentelemetry.context.Scope;
import org.aspectj.lang.ProceedingJoinPoint;
import org.aspectj.lang.annotation.Around;
import org.aspectj.lang.annotation.Aspect;
import org.springframework.boot.autoconfigure.condition.ConditionalOnProperty;
import org.springframework.stereotype.Component;

/**
 * Alternative comprehensive auto-tracing aspect.
 * 
 * Enable this by setting the property: tracing.auto-trace.enabled=true
 * Configure what to trace with additional properties:
 * - tracing.auto-trace.include-private=false (default: only public methods)
 * - tracing.auto-trace.packages=com.example.helloworld (comma-separated packages)
 * 
 * To use this instead of the enhanced TracingAspect:
 * 1. Add the properties to application.properties
 * 2. Comment out the auto-tracing methods in TracingAspect
 */
@Aspect
@Component
@ConditionalOnProperty(name = "tracing.auto-trace.enabled", havingValue = "true", matchIfMissing = false)
public class AutoTracingAspect {

    private final Tracer tracer;

    public AutoTracingAspect(Tracer tracer) {
        this.tracer = tracer;
    }

    /**
     * Traces all public methods in configured packages, excluding:
     * - Methods already annotated with @Traced
     * - Configuration classes
     * - Getters/setters (methods starting with get/set/is)
     * - toString, equals, hashCode methods
     */
    @Around("execution(public * com.example.helloworld..*(..)) && " +
            "!@annotation(com.example.helloworld.Traced) && " +
            "!execution(* com.example.helloworld..*Config*.*(..)) && " +
            "!execution(* get*(..)) && " +
            "!execution(* set*(..)) && " +
            "!execution(* is*(..)) && " +
            "!execution(* toString(..)) && " +
            "!execution(* equals(..)) && " +
            "!execution(* hashCode(..))")
    public Object autoTrace(ProceedingJoinPoint joinPoint) throws Throwable {
        String className = joinPoint.getSignature().getDeclaringType().getSimpleName();
        String methodName = joinPoint.getSignature().getName();
        String spanName = className + "." + methodName;

        Span span = tracer.spanBuilder(spanName)
            .setAttribute("auto.traced", true)
            .setAttribute("class.name", className)
            .setAttribute("method.name", methodName)
            .startSpan();
        
        try (Scope scope = span.makeCurrent()) {
            Object result = joinPoint.proceed();
            
            // Add result information if it's a simple type
            if (result != null) {
                String resultType = result.getClass().getSimpleName();
                span.setAttribute("return.type", resultType);
                
                if (result instanceof String || result instanceof Number || result instanceof Boolean) {
                    span.setAttribute("return.value", result.toString());
                }
            }
            
            span.setStatus(io.opentelemetry.api.trace.StatusCode.OK);
            return result;
        } catch (Exception e) {
            span.recordException(e);
            span.setStatus(io.opentelemetry.api.trace.StatusCode.ERROR, e.getMessage());
            throw e;
        } finally {
            span.end();
        }
    }

    /**
     * Optional: Trace private and protected methods if enabled
     * Enable with: tracing.auto-trace.include-private=true
     */
    @Around("execution(* com.example.helloworld..*(..)) && " +
            "!execution(public * com.example.helloworld..*(..)) && " +
            "!@annotation(com.example.helloworld.Traced) && " +
            "@annotation(org.springframework.boot.autoconfigure.condition.ConditionalOnProperty(name = \"tracing.auto-trace.include-private\", havingValue = \"true\"))")
    @ConditionalOnProperty(name = "tracing.auto-trace.include-private", havingValue = "true", matchIfMissing = false)
    public Object autoTracePrivate(ProceedingJoinPoint joinPoint) throws Throwable {
        String className = joinPoint.getSignature().getDeclaringType().getSimpleName();
        String methodName = joinPoint.getSignature().getName();
        String spanName = className + "." + methodName + " (private)";

        Span span = tracer.spanBuilder(spanName)
            .setAttribute("auto.traced", true)
            .setAttribute("method.visibility", "private")
            .setAttribute("class.name", className)
            .setAttribute("method.name", methodName)
            .startSpan();
        
        try (Scope scope = span.makeCurrent()) {
            return joinPoint.proceed();
        } catch (Exception e) {
            span.recordException(e);
            throw e;
        } finally {
            span.end();
        }
    }
}
