package com.example.oslc;

import com.example.oslc.services.ServiceProviderCatalogService;
import com.example.oslc.services.ServiceProviderService;
import org.glassfish.jersey.server.ResourceConfig;
import org.springframework.stereotype.Component;

@Component
public class JerseyConfig extends ResourceConfig {
    public JerseyConfig() {
        // Register OSLC Providers by package scanning instead of explicit class
        packages("org.eclipse.lyo.oslc4j.provider.jena", "org.eclipse.lyo.oslc4j.provider.json4j");

        // Register custom endpoints
        register(ServiceProviderCatalogService.class);
        register(ServiceProviderService.class);
    }
}
