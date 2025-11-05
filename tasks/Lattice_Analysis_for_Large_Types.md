# Lattice Analysis for Large Types

## Concept

This document outlines a conceptual framework for performing "lattice analysis" on large Rust types, specifically focusing on structs, enums, and functions. The goal is to move beyond simple type usage analysis and understand the deeper structural relationships and co-occurrence patterns within and between these complex types.

The core idea is to treat these large types as nodes in a lattice, where the relationships between their internal components (fields, variants, parameters, return types, and methods within `impl` blocks) form the edges. This analysis aims to identify "knots" or "quasi-fibers" – groups of types that frequently appear together, indicating a cohesive functional or data-oriented unit.

## Motivation

Traditional type usage analysis often focuses on individual type references. However, in large codebases, complex types (structs, enums with many fields/variants, functions with numerous parameters) often represent significant architectural components. Understanding how their internal elements are used in combination can reveal:

*   **Implicit Groupings**: Discovering logical groupings of fields or variants that are consistently accessed or modified together, even if not explicitly grouped in the code.
*   **API Cohesion**: Assessing the cohesion of a type's API by analyzing which parts are frequently used in conjunction.
*   **Refactoring Opportunities**: Identifying areas where types might be too large or where sub-types could be extracted based on co-occurrence patterns. This extends to `impl` blocks, where cohesive groups of methods might suggest extraction into separate traits or helper structs.
*   **Architectural Understanding**: Gaining a deeper insight into the "shape" and "flow" of data and control within the system.

## Scope of Analysis

### 1. Structs as Lattices

A large `struct` can be viewed as a lattice where:
*   **Nodes**: Individual fields of the struct (and their types).
*   **Edges**: Co-occurrence of these fields within expressions, function calls, or other structural contexts.

**Analysis Focus**:
*   **Field Co-occurrence**: Identify groups of 2, 3, 4, 5, or more fields that are frequently accessed or modified together within a given scope (e.g., a function, a method, a block).
*   **Contextual Usage**: Analyze the expressions in which these field groups appear to understand their purpose.
*   **Reporting**: Present these co-occurring groups, their frequency, and the contexts of their usage.

### 2. Enums as Lattices

A large `enum` can be viewed similarly, where:
*   **Nodes**: Types contained within the enum's variants (for tuple or struct variants).
*   **Edges**: Co-occurrence of these variant-contained types within `match` statements, `if let` expressions, or other pattern matching contexts.

**Analysis Focus**:
*   **Variant Type Co-occurrence**: Identify groups of types from different variants that are processed together.
*   **Pattern Matching Contexts**: Analyze the `match` arms or `if let` branches where these types are used.
*   **Reporting**: Highlight common patterns of enum variant usage and the types involved.

### 3. Functions as Lattices

Functions, especially those with many parameters or complex return types, can also form a lattice:
*   **Nodes**: Input parameters (and their types), and components of the return type.
*   **Edges**: Co-occurrence of parameters within the function body, or parameters used in conjunction with parts of the return value.

**Analysis Focus**:
*   **Parameter Co-occurrence**: Identify groups of parameters that are consistently used together within the function's logic.
*   **Parameter-Return Type Relationships**: Analyze how specific input parameters influence specific parts of a complex return type.
*   **Reporting**: Show common parameter groupings and their impact on the function's output.

### 4. Impl Blocks as Lattices of Functions

`impl` blocks, particularly for large structs or enums, can form a lattice of functions:
*   **Nodes**: Individual methods or associated functions within the `impl` block.
*   **Edges**: Dependencies between these methods (e.g., one method calling another), or shared access to the fields/variants of the implemented type.

**Analysis Focus**:
*   **Method Co-occurrence**: Identify groups of methods that are frequently called together or operate on the same subset of the type's data.
*   **Shared State Access**: Analyze which methods access which fields/variants, identifying patterns of shared state manipulation.
*   **Refactoring Opportunities**: Highlight cohesive clusters of methods that could be extracted into traits or smaller, more focused `impl` blocks, thereby "splitting" a larger structure into more manageable components.
*   **Reporting**: Present these method groupings, their dependencies, and their interaction with the underlying type's data.

## Implementation Considerations

*   **AST Traversal**: Extend existing AST visitors (e.g., `TypeUsageVisitor`) to specifically analyze `ItemStruct`, `ItemEnum`, `ItemFn`, and `ItemImpl` nodes.
*   **Co-occurrence Detection**: Develop algorithms to detect and count combinations of types/fields/methods within a defined scope. This might involve:
    *   **Sliding Window**: Analyzing co-occurrence within a certain "window" of AST nodes.
    *   **Graph-based Approaches**: Building a local graph for each large type/impl block and analyzing its subgraphs.
*   **Thresholds**: Define configurable thresholds for what constitutes a "large" type (e.g., number of fields/variants/parameters/methods) and what level of co-occurrence is significant.
*   **Reporting Structure**: The report should be hierarchical, allowing drill-down from the large type/impl block to its internal lattice structure, showing frequencies and code snippets.

## Connection to Broader Lattice Concepts

This micro-lattice analysis of individual types and `impl` blocks can feed into a macro-lattice analysis of modules and flakes. The "knots" identified within a struct's fields, an enum's variants, or an `impl` block's methods could represent fundamental building blocks that are then used as nodes in a higher-level lattice representing inter-module dependencies. This aligns with the idea of a "self-describing and self-contained monster" like Rust, where internal structure informs external relationships.

## Next Steps

1.  **Complete Current Type Usage Analysis**: Ensure the `prelude-generator` is fully functional with its current type usage reporting capabilities.
2.  **Design Data Structures**: Define appropriate data structures to store the co-occurrence information for structs, enums, functions, and `impl` blocks.
3.  **Extend AST Visitors**: Modify `TypeUsageVisitor` or create new visitors to perform the detailed internal analysis of large types and `impl` blocks.
4.  **Integrate Reporting**: Add new sections to the report generator to present the lattice analysis findings.