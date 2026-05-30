package com.example.zoocatalog;

import java.util.List;

public class CatalogTreeResponse {
  private List<CatalogNodeDto> nodes;
  private long totalAnimals;

  public CatalogTreeResponse(List<CatalogNodeDto> nodes, long totalAnimals) {
    this.nodes = nodes;
    this.totalAnimals = totalAnimals;
  }

  public List<CatalogNodeDto> getNodes() {
    return nodes;
  }

  public long getTotalAnimals() {
    return totalAnimals;
  }
}
