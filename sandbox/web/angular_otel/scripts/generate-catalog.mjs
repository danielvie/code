import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const categories = {
  Mammals: {
    Felidae: ['Savanna Ridge', 'Rainforest House', 'Nocturnal Wing'],
    Canidae: ['Prairie Range', 'Temperate Woods', 'Arctic Point'],
    Primates: ['Canopy Walk', 'Island Habitat', 'Research Grove'],
    Ungulates: ['Grassland Loop', 'Highland Yard', 'Wetlands Edge']
  },
  Birds: {
    Raptors: ['Cliff Aviary', 'Desert Flight', 'Forest Perch'],
    Waterfowl: ['Lagoon', 'Marsh Run', 'Reed Pond'],
    Parrots: ['Tropical Dome', 'Canopy Walk', 'Education Wing'],
    Ratites: ['Open Range', 'Scrub Yard', 'Keeper Trail']
  },
  Reptiles: {
    Lizards: ['Desert House', 'Rock Garden', 'Tropical Dome'],
    Snakes: ['Nocturnal Wing', 'Rainforest House', 'Education Wing'],
    Turtles: ['Lagoon', 'Wetlands Edge', 'River Bend'],
    Crocodilians: ['River Bend', 'Marsh Run', 'Tropical Dome']
  },
  Amphibians: {
    Frogs: ['Rainforest House', 'Wetlands Edge', 'Research Grove'],
    Salamanders: ['Temperate Woods', 'River Bend', 'Nocturnal Wing'],
    Newts: ['Marsh Run', 'Education Wing', 'Wetlands Edge']
  },
  Fish: {
    Reef: ['Coral Hall', 'Lagoon', 'Education Wing'],
    Freshwater: ['River Bend', 'Reed Pond', 'Research Grove'],
    Pelagic: ['Ocean Tank', 'Coral Hall', 'Keeper Trail']
  },
  Invertebrates: {
    Arachnids: ['Desert House', 'Nocturnal Wing', 'Education Wing'],
    Insects: ['Rainforest House', 'Research Grove', 'Canopy Walk'],
    Mollusks: ['Ocean Tank', 'Coral Hall', 'Lagoon']
  }
};

const speciesWords = [
  'Golden', 'River', 'Spotted', 'Blue', 'Crowned', 'Mountain', 'Dwarf', 'Giant',
  'Striped', 'Emerald', 'Dusky', 'Scarlet', 'Common', 'Lesser', 'Greater', 'Island'
];
const animalWords = [
  'Lion', 'Wolf', 'Lemur', 'Antelope', 'Eagle', 'Duck', 'Macaw', 'Emu', 'Gecko',
  'Python', 'Turtle', 'Caiman', 'Frog', 'Salamander', 'Newt', 'Tang', 'Trout',
  'Shark', 'Tarantula', 'Beetle', 'Octopus'
];
const diets = ['Herbivore', 'Carnivore', 'Omnivore', 'Piscivore', 'Insectivore', 'Frugivore'];
const statuses = ['Healthy', 'Observation', 'Quarantine', 'Breeding', 'Transfer Review'];
const careLevels = ['Routine', 'Moderate', 'Intensive'];

function pick(values, index, offset = 0) {
  return values[(index + offset) % values.length];
}

function createAnimal(index) {
  const categoryNames = Object.keys(categories);
  const category = pick(categoryNames, index);
  const familyNames = Object.keys(categories[category]);
  const family = pick(familyNames, Math.floor(index / categoryNames.length));
  const habitats = categories[category][family];
  const habitat = pick(habitats, Math.floor(index / (categoryNames.length + familyNames.length)));
  const species = `${pick(speciesWords, index, 3)} ${pick(animalWords, index, family.length)}`;
  const name = `${species} ${String(index + 1).padStart(4, '0')}`;
  const month = String((index % 12) + 1).padStart(2, '0');
  const day = String((index % 27) + 1).padStart(2, '0');

  return {
    id: `animal-${String(index + 1).padStart(5, '0')}`,
    name,
    species,
    category,
    family,
    habitat,
    diet: pick(diets, index, family.length),
    status: pick(statuses, index, habitat.length),
    careLevel: pick(careLevels, index, species.length),
    lastCheckup: `2026-${month}-${day}`,
    tag: `ZOO-${category.slice(0, 3).toUpperCase()}-${String(index % 9000).padStart(4, '0')}`
  };
}

function writeCatalog(filename, size) {
  const animals = Array.from({ length: size }, (_, index) => createAnimal(index));
  writeFileSync(join('src', 'assets', filename), `${JSON.stringify(animals, null, 2)}\n`);
}

mkdirSync(join('src', 'assets'), { recursive: true });
writeCatalog('catalog-small.json', 180);
writeCatalog('catalog-large.json', 6500);
