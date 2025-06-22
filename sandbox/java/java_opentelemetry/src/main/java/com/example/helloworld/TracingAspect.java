package com.example.helloworld;

import io.opentelemetry.api.trace.Span;
import io.opentelemetry.api.trace.Tracer;
import io.opentelemetry.context.Scope;
import org.aspectj.lang.ProceedingJoinPoint;
import org.aspectj.lang.annotation.Around;
import org.aspectj.lang.annotation.Aspect;
import org.springframework.stereotype.Component;

@Aspect
@Component
public class TracingAspect {

    private final Tracer tracer;

    public TracingAspect(Tracer tracer) {
        this.tracer = tracer;
    }

    // Existing annotation-based tracing
    @Around("@annotation(traced)")
    public Object trace(ProceedingJoinPoint joinPoint, Traced traced) throws Throwable {
        String spanName = traced.value().isEmpty() ? joinPoint.getSignature().getName() : traced.value();

        Span span = tracer.spanBuilder(spanName).startSpan();
        try (Scope scope = span.makeCurrent()) {
            return joinPoint.proceed();
        } catch (Exception e) {
            span.recordException(e);
            throw e;
        } finally {
            span.end();
        }
    }

    // Automatic tracing for all public methods in service and controller classes
    @Around("(execution(public * com.example.helloworld..*Service*.*(..)) || " +
            "execution(public * com.example.helloworld..*Controller*.*(..))) && " +
            "!@annotation(com.example.helloworld.Traced)")
    public Object traceServiceAndControllerMethods(ProceedingJoinPoint joinPoint) throws Throwable {
        String className = joinPoint.getSignature().getDeclaringType().getSimpleName();
        String methodName = joinPoint.getSignature().getName();
        String spanName = className + "." + methodName;

        Span span = tracer.spanBuilder(spanName)
            .setAttribute("class.name", className)
            .setAttribute("method.name", methodName)
            .startSpan();
          try (Scope scope = span.makeCurrent()) {
            Object result = joinPoint.proceed();
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
}
