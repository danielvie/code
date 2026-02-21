package com.example.oslc.services;

import com.example.oslc.resources.Defect;
import org.eclipse.lyo.oslc4j.core.annotation.OslcCreationFactory;
import org.eclipse.lyo.oslc4j.core.annotation.OslcQueryCapability;
import org.eclipse.lyo.oslc4j.core.annotation.OslcService;
import org.eclipse.lyo.oslc4j.core.model.ServiceProvider;
import org.springframework.stereotype.Service;

import jakarta.ws.rs.Consumes;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.PathParam;
import jakarta.ws.rs.Produces;
import jakarta.ws.rs.core.Context;
import jakarta.ws.rs.core.Response;
import jakarta.ws.rs.core.UriInfo;
import java.net.URI;
import java.net.URISyntaxException;
import java.util.ArrayList;
import java.util.Date;
import java.util.List;
import java.util.UUID;

@Service
@Path("/provider/{id}")
@OslcService("http://open-services.net/ns/cm#")
public class ServiceProviderService {

    private static final List<Defect> defects = new ArrayList<>();

    @GET
    @Produces({"application/rdf+xml", "application/json-ld", "application/xml"})
    public ServiceProvider getServiceProvider(@Context UriInfo uriInfo, @PathParam("id") String id) throws URISyntaxException {
        ServiceProvider provider = new ServiceProvider();
        provider.setAbout(uriInfo.getAbsolutePath());
        provider.setTitle("Service Provider " + id);
        provider.setDescription("A Service Provider for the OSLC PoC");
        return provider;
    }

    @GET
    @Path("defects")
    @Produces({"application/rdf+xml", "application/json-ld", "application/xml"})
    @OslcQueryCapability(title = "Defect Query Capability", label = "Defect Query",
            resourceShape = "http://open-services.net/ns/cm#Defect",
            resourceTypes = {"http://open-services.net/ns/cm#Defect"}, usages = {"http://open-services.net/ns/core#default"})
    public List<Defect> queryDefects() {
        return defects; // A real implementation would filter based on OSLC query parameters
    }

    @POST
    @Path("defects")
    @Consumes({"application/rdf+xml", "application/json-ld", "application/xml"})
    @Produces({"application/rdf+xml", "application/json-ld", "application/xml"})
    @OslcCreationFactory(title = "Defect Creation Factory", label = "Defect Creation",
            resourceShapes = {"http://open-services.net/ns/cm#Defect"},
            resourceTypes = {"http://open-services.net/ns/cm#Defect"}, usages = {"http://open-services.net/ns/core#default"})
    public Response createDefect(Defect defect, @Context UriInfo uriInfo) throws URISyntaxException {
        defect.setIdentifier(UUID.randomUUID().toString());
        defect.setCreated(new Date());
        defect.setAbout(new URI(uriInfo.getAbsolutePath().toString() + "/" + defect.getIdentifier()));
        defects.add(defect);
        return Response.created(defect.getAbout()).entity(defect).build();
    }
}
