package com.example.zoocatalog;

import javax.persistence.Entity;
import javax.persistence.Id;

@Entity
public class CatalogNode {
  @Id
  private String id;
  private String parentId;
  private String label;
  private String type;
  private int itemCount;
  private int sortOrder;
  private String name;
  private String species;
  private String category;
  private String family;
  private String habitat;
  private String diet;
  private String status;
  private String careLevel;
  private String lastCheckup;
  private String tag;

  public String getId() {
    return id;
  }

  public void setId(String id) {
    this.id = id;
  }

  public String getParentId() {
    return parentId;
  }

  public void setParentId(String parentId) {
    this.parentId = parentId;
  }

  public String getLabel() {
    return label;
  }

  public void setLabel(String label) {
    this.label = label;
  }

  public String getType() {
    return type;
  }

  public void setType(String type) {
    this.type = type;
  }

  public int getItemCount() {
    return itemCount;
  }

  public void setItemCount(int itemCount) {
    this.itemCount = itemCount;
  }

  public int getSortOrder() {
    return sortOrder;
  }

  public void setSortOrder(int sortOrder) {
    this.sortOrder = sortOrder;
  }

  public String getName() {
    return name;
  }

  public void setName(String name) {
    this.name = name;
  }

  public String getSpecies() {
    return species;
  }

  public void setSpecies(String species) {
    this.species = species;
  }

  public String getCategory() {
    return category;
  }

  public void setCategory(String category) {
    this.category = category;
  }

  public String getFamily() {
    return family;
  }

  public void setFamily(String family) {
    this.family = family;
  }

  public String getHabitat() {
    return habitat;
  }

  public void setHabitat(String habitat) {
    this.habitat = habitat;
  }

  public String getDiet() {
    return diet;
  }

  public void setDiet(String diet) {
    this.diet = diet;
  }

  public String getStatus() {
    return status;
  }

  public void setStatus(String status) {
    this.status = status;
  }

  public String getCareLevel() {
    return careLevel;
  }

  public void setCareLevel(String careLevel) {
    this.careLevel = careLevel;
  }

  public String getLastCheckup() {
    return lastCheckup;
  }

  public void setLastCheckup(String lastCheckup) {
    this.lastCheckup = lastCheckup;
  }

  public String getTag() {
    return tag;
  }

  public void setTag(String tag) {
    this.tag = tag;
  }
}
