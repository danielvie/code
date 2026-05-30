package com.example.zoocatalog;

import java.time.LocalDate;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import org.springframework.stereotype.Component;

@Component
public class CatalogGenerator {
  private final Map<String, Map<String, List<String>>> categories = new LinkedHashMap<>();
  private final List<String> speciesWords = List.of(
      "Golden", "River", "Spotted", "Blue", "Crowned", "Mountain", "Dwarf", "Giant",
      "Striped", "Emerald", "Dusky", "Scarlet", "Common", "Lesser", "Greater", "Island");
  private final List<String> animalWords = List.of(
      "Lion", "Wolf", "Lemur", "Antelope", "Eagle", "Duck", "Macaw", "Emu", "Gecko",
      "Python", "Turtle", "Caiman", "Frog", "Salamander", "Newt", "Tang", "Trout",
      "Shark", "Tarantula", "Beetle", "Octopus");
  private final List<String> diets = List.of("Herbivore", "Carnivore", "Omnivore", "Piscivore", "Insectivore", "Frugivore");
  private final List<String> statuses = List.of("Healthy", "Observation", "Quarantine", "Breeding", "Transfer Review");
  private final List<String> careLevels = List.of("Routine", "Moderate", "Intensive");

  public CatalogGenerator() {
    categories.put("Mammals", Map.of(
        "Felidae", List.of("Savanna Ridge", "Rainforest House", "Nocturnal Wing"),
        "Canidae", List.of("Prairie Range", "Temperate Woods", "Arctic Point"),
        "Primates", List.of("Canopy Walk", "Island Habitat", "Research Grove"),
        "Ungulates", List.of("Grassland Loop", "Highland Yard", "Wetlands Edge")));
    categories.put("Birds", Map.of(
        "Raptors", List.of("Cliff Aviary", "Desert Flight", "Forest Perch"),
        "Waterfowl", List.of("Lagoon", "Marsh Run", "Reed Pond"),
        "Parrots", List.of("Tropical Dome", "Canopy Walk", "Education Wing"),
        "Ratites", List.of("Open Range", "Scrub Yard", "Keeper Trail")));
    categories.put("Reptiles", Map.of(
        "Lizards", List.of("Desert House", "Rock Garden", "Tropical Dome"),
        "Snakes", List.of("Nocturnal Wing", "Rainforest House", "Education Wing"),
        "Turtles", List.of("Lagoon", "Wetlands Edge", "River Bend"),
        "Crocodilians", List.of("River Bend", "Marsh Run", "Tropical Dome")));
    categories.put("Amphibians", Map.of(
        "Frogs", List.of("Rainforest House", "Wetlands Edge", "Research Grove"),
        "Salamanders", List.of("Temperate Woods", "River Bend", "Nocturnal Wing"),
        "Newts", List.of("Marsh Run", "Education Wing", "Wetlands Edge")));
    categories.put("Fish", Map.of(
        "Reef", List.of("Coral Hall", "Lagoon", "Education Wing"),
        "Freshwater", List.of("River Bend", "Reed Pond", "Research Grove"),
        "Pelagic", List.of("Ocean Tank", "Coral Hall", "Keeper Trail")));
    categories.put("Invertebrates", Map.of(
        "Arachnids", List.of("Desert House", "Nocturnal Wing", "Education Wing"),
        "Insects", List.of("Rainforest House", "Research Grove", "Canopy Walk"),
        "Mollusks", List.of("Ocean Tank", "Coral Hall", "Lagoon")));
  }

  public List<Animal> create(String dataset) {
    int size = "large".equalsIgnoreCase(dataset) ? 6500 : 180;
    List<Animal> animals = new ArrayList<>();

    for (int i = 0; i < size; i++) {
      animals.add(createAnimal(i));
    }

    return animals;
  }

  private Animal createAnimal(int index) {
    List<String> categoryNames = new ArrayList<>(categories.keySet());
    String category = pick(categoryNames, index, 0);
    List<String> familyNames = new ArrayList<>(categories.get(category).keySet());
    String family = pick(familyNames, index / categoryNames.size(), 0);
    List<String> habitats = categories.get(category).get(family);
    String habitat = pick(habitats, index / (categoryNames.size() + familyNames.size()), 0);
    String species = pick(speciesWords, index, 3) + " " + pick(animalWords, index, family.length());
    String name = species + " " + String.format("%04d", index + 1);
    LocalDate checkup = LocalDate.of(2026, (index % 12) + 1, (index % 27) + 1);

    return new Animal(
        "animal-" + String.format("%05d", index + 1),
        name,
        species,
        category,
        family,
        habitat,
        pick(diets, index, family.length()),
        pick(statuses, index, habitat.length()),
        pick(careLevels, index, species.length()),
        checkup.toString(),
        "ZOO-" + category.substring(0, 3).toUpperCase() + "-" + String.format("%04d", index % 9000));
  }

  private String pick(List<String> values, int index, int offset) {
    return values.get((index + offset) % values.size());
  }
}
