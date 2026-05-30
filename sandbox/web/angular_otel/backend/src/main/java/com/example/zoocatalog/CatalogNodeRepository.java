package com.example.zoocatalog;

import java.util.List;
import org.springframework.data.jpa.repository.JpaRepository;

public interface CatalogNodeRepository extends JpaRepository<CatalogNode, String> {
  List<CatalogNode> findByParentIdOrderBySortOrderAsc(String parentId);

  List<CatalogNode> findByParentIdIsNullOrderBySortOrderAsc();

  boolean existsByParentId(String parentId);

  long countByType(String type);
}
