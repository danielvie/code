package com.example.oslc.services;

import com.example.oslc.resources.Defect;
import org.eclipse.lyo.oslc4j.core.annotation.OslcCreationFactory;
import org.eclipse.lyo.oslc4j.core.annotation.OslcQueryCapability;
import org.eclipse.lyo.oslc4j.core.annotation.OslcService;
import org.eclipse.lyo.oslc4j.core.model.ServiceProvider;
import org.springframework.stereotype.Service;

import javax.ws.rs.Consumes;
import javax.ws.rs.DELETE;
import javax.ws.rs.GET;
import javax.ws.rs.POST;
import javax.ws.rs.PUT;
import javax.ws.rs.Path;
import javax.ws.rs.PathParam;
import javax.ws.rs.Produces;
import javax.ws.rs.core.Context;
import javax.ws.rs.core.Response;
import javax.ws.rs.core.UriInfo;
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
    @Produces({"application/rdf+xml", "application/ld+json", "application/xml", "application/json"})
    public ServiceProvider getServiceProvider(@Context UriInfo uriInfo, @PathParam("id") String id) throws URISyntaxException {
        ServiceProvider provider = new ServiceProvider();
        provider.setAbout(uriInfo.getAbsolutePath());
        provider.setTitle("Service Provider " + id);
        provider.setDescription("A Service Provider for the OSLC PoC");
        return provider;
    }

    @GET
    @Path("defects")
    @Produces({"application/rdf+xml", "application/ld+json", "application/xml", "application/json"})
    @OslcQueryCapability(title = "Defect Query Capability", label = "Defect Query",
            resourceShape = "http://open-services.net/ns/cm#Defect",
            resourceTypes = {"http://open-services.net/ns/cm#Defect"}, usages = {"http://open-services.net/ns/core#default"})
    public List<Defect> queryDefects() {
        return defects; // A real implementation would filter based on OSLC query parameters
    }

    @POST
    @Path("defects")
    @Consumes({"application/rdf+xml", "application/ld+json", "application/xml", "application/json"})
    @Produces({"application/rdf+xml", "application/ld+json", "application/xml", "application/json"})
    @OslcCreationFactory(title = "Defect Creation Factory", label = "Defect Creation",
            resourceShapes = {"http://open-services.net/ns/cm#Defect"},
            resourceTypes = {"http://open-services.net/ns/cm#Defect"}, usages = {"http://open-services.net/ns/core#default"})
    public Response createDefect(@PathParam("id") String id, Defect defect, @Context UriInfo uriInfo) throws URISyntaxException {
        defect.setIdentifier(UUID.randomUUID().toString());
        defect.setCreated(new Date());
        defect.setAbout(new URI(uriInfo.getAbsolutePath().toString() + "/" + defect.getIdentifier()));
        defects.add(defect);
        return Response.created(defect.getAbout()).entity(defect).build();
    }

    @DELETE
    @Path("defects")
    @Produces({"application/json"})
    public Response deleteAllDefects(@PathParam("id") String id) {
        defects.clear();
        return Response.noContent().build();
    }

    @PUT
    @Path("defects/{defectId}")
    @Consumes({"application/rdf+xml", "application/ld+json", "application/xml", "application/json"})
    @Produces({"application/rdf+xml", "application/ld+json", "application/xml", "application/json"})
    public Response updateDefect(@PathParam("id") String id, @PathParam("defectId") String defectId, Defect updatedDefect) {
        for (int i = 0; i < defects.size(); i++) {
            Defect defect = defects.get(i);
            if (defect.getIdentifier().equals(defectId)) {
                defect.setTitle(updatedDefect.getTitle());
                defect.setDescription(updatedDefect.getDescription());
                return Response.ok(defect).build();
            }
        }
        return Response.status(Response.Status.NOT_FOUND).build();
    }

    @DELETE
    @Path("defects/{defectId}")
    @Produces({"application/json"})
    public Response deleteDefect(@PathParam("id") String id, @PathParam("defectId") String defectId) {
        boolean removed = defects.removeIf(d -> d.getIdentifier().equals(defectId));
        if (removed) {
            return Response.noContent().build();
        }
        return Response.status(Response.Status.NOT_FOUND).build();
    }
}
