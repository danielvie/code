package com.example.helloworld;

import io.opentelemetry.api.trace.Span;
import io.opentelemetry.api.trace.Tracer;
import io.opentelemetry.context.Scope;

import java.util.function.Supplier;

/**
 * Utility class for common tracing patterns.
 * Reduces boilerplate code when manual tracing is needed.
 */
public class TracingUtils {

    /**
     * Execute an operation within a named span.
     * Handles span creation, scope management, exception recording, and cleanup.
     */
    public static <T> T withSpan(Tracer tracer, String spanName, Supplier<T> operation) {
        Span span = tracer.spanBuilder(spanName).startSpan();
        try (Scope scope = span.makeCurrent()) {
            return operation.get();
        } catch (Exception e) {
            span.recordException(e);
            throw e;
        } finally {
            span.end();
        }
    }

    /**
     * Execute a void operation within a named span.
     */
    public static void withSpan(Tracer tracer, String spanName, Runnable operation) {
        Span span = tracer.spanBuilder(spanName).startSpan();
        try (Scope scope = span.makeCurrent()) {
            operation.run();
        } catch (Exception e) {
            span.recordException(e);
            throw e;
        } finally {
            span.end();
        }
    }

    /**
     * Execute an operation that may throw checked exceptions within a named span.
     */
    public static <T, E extends Exception> T withSpanThrows(
            Tracer tracer, 
            String spanName, 
            ThrowingSupplier<T, E> operation) throws E {
        Span span = tracer.spanBuilder(spanName).startSpan();
        try (Scope scope = span.makeCurrent()) {
            return operation.get();
        } catch (Exception e) {
            span.recordException(e);
            throw e;
        } finally {
            span.end();
        }
    }

    @FunctionalInterface
    public interface ThrowingSupplier<T, E extends Exception> {
        T get() throws E;
    }
}
