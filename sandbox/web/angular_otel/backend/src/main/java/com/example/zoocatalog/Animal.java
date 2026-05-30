package com.example.zoocatalog;

import javax.validation.constraints.NotBlank;

public class Animal {
  private String id;

  @NotBlank
  private String name;

  @NotBlank
  private String species;

  @NotBlank
  private String category;

  @NotBlank
  private String family;

  @NotBlank
  private String habitat;

  private String diet;
  private String status;
  private String careLevel;
  private String lastCheckup;
  private String tag;

  public Animal() {
  }

  public Animal(
      String id,
      String name,
      String species,
      String category,
      String family,
      String habitat,
      String diet,
      String status,
      String careLevel,
      String lastCheckup,
      String tag) {
    this.id = id;
    this.name = name;
    this.species = species;
    this.category = category;
    this.family = family;
    this.habitat = habitat;
    this.diet = diet;
    this.status = status;
    this.careLevel = careLevel;
    this.lastCheckup = lastCheckup;
    this.tag = tag;
  }

  public String getId() {
    return id;
  }

  public void setId(String id) {
    this.id = id;
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
