package com.example.zoocatalog;

import javax.validation.Valid;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api")
@CrossOrigin(origins = "http://localhost:4200")
public class CatalogController {
  private final CatalogService catalogService;

  public CatalogController(CatalogService catalogService) {
    this.catalogService = catalogService;
  }

  @GetMapping("/animals")
  public CatalogTreeResponse roots() {
    return catalogService.roots();
  }

  @PostMapping("/catalog/load")
  public CatalogTreeResponse loadDataset(@RequestBody DatasetRequest request) {
    return catalogService.loadDataset(request.getDataset());
  }

  @GetMapping("/nodes/{id}")
  public CatalogNodeDto getNode(@PathVariable String id) {
    return catalogService.getNode(id);
  }

  @PostMapping("/animals")
  public Animal create(@Valid @RequestBody Animal animal) {
    return catalogService.create(animal);
  }

  @PutMapping("/animals/{id}")
  public Animal update(@PathVariable String id, @Valid @RequestBody Animal animal) {
    return catalogService.update(id, animal);
  }

  @DeleteMapping("/animals/{id}")
  public void delete(@PathVariable String id) {
    catalogService.delete(id);
  }
}
