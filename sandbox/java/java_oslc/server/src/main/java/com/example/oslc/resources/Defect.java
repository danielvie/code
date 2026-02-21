package com.example.oslc.resources;

import org.eclipse.lyo.oslc4j.core.annotation.OslcDescription;
import org.eclipse.lyo.oslc4j.core.annotation.OslcName;
import org.eclipse.lyo.oslc4j.core.annotation.OslcNamespace;
import org.eclipse.lyo.oslc4j.core.annotation.OslcPropertyDefinition;
import org.eclipse.lyo.oslc4j.core.annotation.OslcResourceShape;
import org.eclipse.lyo.oslc4j.core.annotation.OslcTitle;
import org.eclipse.lyo.oslc4j.core.model.AbstractResource;

import java.net.URI;
import java.util.Date;

@OslcNamespace("http://open-services.net/ns/cm#")
@OslcName("Defect")
@OslcResourceShape(title = "Defect Resource Shape", describes = "http://open-services.net/ns/cm#Defect")
public class Defect extends AbstractResource {

    private String identifier;
    private String title;
    private String description;
    private String status;
    private Date created;

    public Defect() {
        super();
    }

    public Defect(URI about) {
        super(about);
    }

    @OslcDescription("The unique identifier")
    @OslcPropertyDefinition("http://purl.org/dc/terms/identifier")
    @OslcTitle("Identifier")
    public String getIdentifier() {
        return identifier;
    }

    public void setIdentifier(String identifier) {
        this.identifier = identifier;
    }

    @OslcDescription("Title or summary of the defect")
    @OslcPropertyDefinition("http://purl.org/dc/terms/title")
    @OslcTitle("Title")
    public String getTitle() {
        return title;
    }

    public void setTitle(String title) {
        this.title = title;
    }

    @OslcDescription("Detailed description of the defect")
    @OslcPropertyDefinition("http://purl.org/dc/terms/description")
    @OslcTitle("Description")
    public String getDescription() {
        return description;
    }

    public void setDescription(String description) {
        this.description = description;
    }

    @OslcDescription("Status of the defect")
    @OslcPropertyDefinition("http://open-services.net/ns/cm#status")
    @OslcTitle("Status")
    public String getStatus() {
        return status;
    }

    public void setStatus(String status) {
        this.status = status;
    }

    @OslcDescription("Creation date")
    @OslcPropertyDefinition("http://purl.org/dc/terms/created")
    @OslcTitle("Created")
    public Date getCreated() {
        return created;
    }

    public void setCreated(Date created) {
        this.created = created;
    }
}
