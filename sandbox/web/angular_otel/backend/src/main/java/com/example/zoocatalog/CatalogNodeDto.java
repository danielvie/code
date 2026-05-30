package com.example.zoocatalog;

import java.util.List;

public class CatalogNodeDto {
  private String key;
  private String label;
  private String type;
  private int count;
  private boolean hasChildren;
  private Animal animal;
  private List<CatalogNodeDto> children;

  public CatalogNodeDto(String key, String label, String type, int count, boolean hasChildren, Animal animal, List<CatalogNodeDto> children) {
    this.key = key;
    this.label = label;
    this.type = type;
    this.count = count;
    this.hasChildren = hasChildren;
    this.animal = animal;
    this.children = children;
  }

  public String getKey() {
    return key;
  }

  public String getLabel() {
    return label;
  }

  public String getType() {
    return type;
  }

  public int getCount() {
    return count;
  }

  public boolean isHasChildren() {
    return hasChildren;
  }

  public Animal getAnimal() {
    return animal;
  }

  public List<CatalogNodeDto> getChildren() {
    return children;
  }
}
