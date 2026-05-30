package com.example.zoocatalog;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import java.util.stream.Collectors;
import org.springframework.http.HttpStatus;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import org.springframework.web.server.ResponseStatusException;

@Service
public class CatalogService {
  private final CatalogGenerator generator;
  private final CatalogNodeRepository repository;
  private final Map<String, List<CatalogNodeDto>> requestLog = new LinkedHashMap<>();
  private int sortOrder = 0;

  public CatalogService(CatalogGenerator generator, CatalogNodeRepository repository) {
    this.generator = generator;
    this.repository = repository;
    loadDataset("small");
  }

  @Transactional
  public CatalogTreeResponse loadDataset(String dataset) {
    repository.deleteAll();
    sortOrder = 0;
    Map<String, CatalogNode> groups = new HashMap<>();

    for (Animal animal : generator.create(dataset)) {
      CatalogNode category = group(groups, null, "category", animal.getCategory(), animal.getCategory());
      CatalogNode family = group(groups, category.getId(), "family", animal.getFamily(), animal.getFamily());
      CatalogNode parent = family;

      if (Math.abs(animal.getTag().hashCode()) % 4 != 0) {
        parent = group(groups, family.getId(), "habitat", animal.getHabitat(), animal.getHabitat());
      }

      if (Math.abs(animal.getId().hashCode()) % 5 == 0) {
        parent = group(groups, parent.getId(), "enclosure", "Sector " + ((animal.getTag().hashCode() & 3) + 1), animal.getHabitat());
      }

      CatalogNode animalNode = new CatalogNode();
      animalNode.setId(animal.getId());
      animalNode.setParentId(parent.getId());
      animalNode.setLabel(animal.getName());
      animalNode.setType("animal");
      animalNode.setItemCount(0);
      animalNode.setSortOrder(sortOrder++);
      copyAnimal(animal, animalNode);
      repository.save(animalNode);
    }

    for (CatalogNode node : repository.findAll()) {
      if (!"animal".equals(node.getType())) {
        node.setItemCount(countAnimalsBelow(node.getId()));
        repository.save(node);
      }
    }

    return roots();
  }

  public CatalogTreeResponse roots() {
    return new CatalogTreeResponse(
        repository.findByParentIdIsNullOrderBySortOrderAsc().stream().map(this::summary).collect(Collectors.toList()),
        repository.countByType("animal"));
  }

  public CatalogNodeDto getNode(String id) {
    CatalogNode node = findNode(id);
    List<CatalogNodeDto> children = repository.findByParentIdOrderBySortOrderAsc(id).stream()
        .map(this::summary)
        .collect(Collectors.toList());

    requestLog.put(id + "-" + System.nanoTime(), children);
    return dto(node, children);
  }

  public Animal create(Animal animal) {
    CatalogNode category = group(new HashMap<>(), null, "category", animal.getCategory(), animal.getCategory());
    CatalogNode family = group(new HashMap<>(), category.getId(), "family", animal.getFamily(), animal.getFamily());
    CatalogNode habitat = group(new HashMap<>(), family.getId(), "habitat", animal.getHabitat(), animal.getHabitat());
    CatalogNode animalNode = new CatalogNode();
    animalNode.setId("animal-" + UUID.randomUUID());
    animalNode.setParentId(habitat.getId());
    animalNode.setLabel(defaultValue(animal.getName(), "New Zoo Animal"));
    animalNode.setType("animal");
    animalNode.setSortOrder(sortOrder++);
    copyAnimal(animal, animalNode);
    animalNode.setName(defaultValue(animalNode.getName(), animalNode.getLabel()));
    repository.save(animalNode);
    return toAnimal(animalNode);
  }

  public Animal update(String id, Animal update) {
    CatalogNode node = findNode(id);
    if (!"animal".equals(node.getType())) {
      throw new ResponseStatusException(HttpStatus.BAD_REQUEST, "Only animal nodes can be updated");
    }

    copyAnimal(update, node);
    node.setLabel(update.getName());
    repository.save(node);
    return toAnimal(node);
  }

  public void delete(String id) {
    CatalogNode node = findNode(id);
    repository.delete(node);
  }

  private CatalogNode group(Map<String, CatalogNode> groups, String parentId, String type, String label, String categoryValue) {
    String key = (parentId == null ? "root" : parentId) + "/" + type + "/" + label;
    if (groups.containsKey(key)) {
      return groups.get(key);
    }

    List<CatalogNode> siblings = parentId == null
        ? repository.findByParentIdIsNullOrderBySortOrderAsc()
        : repository.findByParentIdOrderBySortOrderAsc(parentId);
    for (CatalogNode sibling : siblings) {
      if (sibling.getType().equals(type) && sibling.getLabel().equals(label)) {
        groups.put(key, sibling);
        return sibling;
      }
    }

    CatalogNode node = new CatalogNode();
    node.setId(type + "-" + UUID.randomUUID());
    node.setParentId(parentId);
    node.setLabel(label);
    node.setType(type);
    node.setCategory(categoryValue);
    node.setSortOrder(sortOrder++);
    repository.save(node);
    groups.put(key, node);
    return node;
  }

  private int countAnimalsBelow(String id) {
    int count = 0;
    for (CatalogNode child : repository.findByParentIdOrderBySortOrderAsc(id)) {
      if ("animal".equals(child.getType())) {
        count += 1;
      } else {
        count += countAnimalsBelow(child.getId());
      }
    }
    return count;
  }

  private CatalogNodeDto summary(CatalogNode node) {
    return dto(node, List.of());
  }

  private CatalogNodeDto dto(CatalogNode node, List<CatalogNodeDto> children) {
    return new CatalogNodeDto(
        node.getId(),
        node.getLabel(),
        node.getType(),
        node.getItemCount(),
        repository.existsByParentId(node.getId()),
        "animal".equals(node.getType()) ? toAnimal(node) : null,
        children);
  }

  private CatalogNode findNode(String id) {
    return repository.findById(id)
        .orElseThrow(() -> new ResponseStatusException(HttpStatus.NOT_FOUND, "Catalog node not found"));
  }

  private void copyAnimal(Animal animal, CatalogNode node) {
    node.setName(defaultValue(animal.getName(), node.getLabel()));
    node.setSpecies(defaultValue(animal.getSpecies(), "Unknown Species"));
    node.setCategory(defaultValue(animal.getCategory(), "Mammals"));
    node.setFamily(defaultValue(animal.getFamily(), "Unassigned"));
    node.setHabitat(defaultValue(animal.getHabitat(), "Unassigned"));
    node.setDiet(defaultValue(animal.getDiet(), "Omnivore"));
    node.setStatus(defaultValue(animal.getStatus(), "Observation"));
    node.setCareLevel(defaultValue(animal.getCareLevel(), "Routine"));
    node.setLastCheckup(defaultValue(animal.getLastCheckup(), "2026-01-01"));
    node.setTag(defaultValue(animal.getTag(), "ZOO-CUSTOM-" + node.getId()));
  }

  private Animal toAnimal(CatalogNode node) {
    return new Animal(
        node.getId(),
        node.getName(),
        node.getSpecies(),
        node.getCategory(),
        node.getFamily(),
        node.getHabitat(),
        node.getDiet(),
        node.getStatus(),
        node.getCareLevel(),
        node.getLastCheckup(),
        node.getTag());
  }

  private String defaultValue(String value, String fallback) {
    return value == null || value.isBlank() ? fallback : value;
  }
}
