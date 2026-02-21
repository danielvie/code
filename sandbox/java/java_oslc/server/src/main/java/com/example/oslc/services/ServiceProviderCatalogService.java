package com.example.oslc.services;

import org.eclipse.lyo.oslc4j.core.model.Publisher;
import org.eclipse.lyo.oslc4j.core.model.ServiceProviderCatalog;
import org.springframework.stereotype.Service;

import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.Context;
import javax.ws.rs.core.UriInfo;
import java.net.URI;
import java.net.URISyntaxException;

@Service
@Path("/catalog")
public class ServiceProviderCatalogService {

    @GET
    @Produces({"application/rdf+xml", "application/ld+json", "application/xml"})
    public ServiceProviderCatalog getCatalog(@Context UriInfo uriInfo) throws URISyntaxException {
        ServiceProviderCatalog catalog = new ServiceProviderCatalog();
        catalog.setAbout(uriInfo.getAbsolutePath());
        catalog.setTitle("OSLC OSLC Proof of Concept Catalog");
        catalog.setDescription("A Catalog of Service Providers for the OSLC PoC");
        
        Publisher publisher = new Publisher("Example Corporation", "com.example");
        publisher.setIcon(new URI("http://example.com/icon.png"));
        catalog.setPublisher(publisher);

        return catalog;
    }
}
