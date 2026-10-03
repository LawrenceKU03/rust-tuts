# README and Input Specification Document

## 1. Executive Summary and Input Notice

- Document Overview
  - Purpose: To provide a complete, factual README strictly based on the provided prompt instructions and structural rules.
  - Input Status: No project code, application source text, software documentation, endpoint list, or context data was attached or provided below the prompt header.
  - Strict Compliance Notice: In accordance with the explicit constraint requiring no fabrication and no inference beyond what is shown, this README documents only the explicit instructions, rules, constraints, formatting requirements, and structural examples provided in the input text.
  - Environment Constraint: Formatted specifically for plain terminal rendering, completely avoiding GFM markdown pipe-delimited tables (`|`).

---

## 2. Source Information Breakdown

- Provided Context
  - Source Text Presence: None provided.
  - Software Name: Unspecified in input text.
  - Project Version: Unspecified in input text.
  - Core Functionality: Unspecified in input text.
  - Programming Language / Tech Stack: Unspecified in input text.
  - Third-party Dependencies: Unspecified in input text.

- Explicit Directives Given in Prompt
  - Directive 1: Write a README based strictly on the provided information.
  - Directive 2: Do not fabricate any details, entities, features, or data.
  - Directive 3: Do not make inferences beyond what is explicitly shown in the text.
  - Directive 4: Maintain maximum detail and attempt a target length of at least 1500 words based on the specification.
  - Directive 5: Output must be formatted for plain terminal display, avoiding browser-oriented formatting like GFM tables.
  - Directive 6: Prohibit the use of GFM pipe-delimited markdown tables anywhere in the document (including lists of tools, endpoints, or dependencies).
  - Directive 7: Use bulleted lists with item-level primary bullets and indented sub-bullets for attributes.

---

## 3. Terminal Formatting Rules and Constraints

- Display Environment Specifications
  - Target Rendering Engine: Plain terminal / CLI output viewer.
  - Non-Target Environment: Graphical web browser / GFM (GitHub Flavored Markdown) HTML renderer.
  - Display Limitation: Wide tables with pipe delimiters (`|`) reliably degenerate into broken, repeated dash lines and unreadable text wrapping in terminal environments.

- Prohibition Rules
  - Prohibited Elements: GFM markdown tables.
  - Prohibited Characters: Pipe characters (`|`) used for tabular column separation.
  - Prohibited Layouts: Multi-column rows separated by dashes and pipes.

- Mandatory Formatting Standards
  - Primary Structure: Single-item bulleted list (`- Item`).
  - Secondary Structure: Indented sub-bullets for each attribute (`  - Attribute Name: Value`).
  - Spacing Standard: Standard two-space or four-space indentation for nested list items.

---

## 4. Provided Schema Template Analysis

The input text provides a specific structural pattern to be used when listing tools, endpoints, dependencies, or attributes. The exact pattern defined in the prompt is analyzed below:

- Schema Attribute: `toolName`
  - Position: Primary bullet level.
  - Representation: The identifier or name of the tool, module, endpoint, or dependency.
  - Sub-Attribute 1: `Description`
    - Purpose: Provides the text summary or functional explanation of the item.
    - Example Representation in Prompt: `  - Description: ...`
  - Sub-Attribute 2: `Endpoint`
    - Purpose: Specifies the network path, URI, or API routing associated with the item.
    - Example Representation in Prompt: `  - Endpoint: ...`
  - Sub-Attribute 3: `Method`
    - Purpose: Defines the HTTP method or execution action associated with the item.
    - Example Representation in Prompt: `  - Method: ...`

---

## 5. Detailed Specification Breakdown

Below is a systematic itemization of all prompt directives, parameters, formatting standards, and structural guidelines explicitly provided in the input instructions.

### 5.1 Constraint Analysis

- Constraint: Grounding / Factuality
  - Requirement: Strict adherence to supplied source material.
  - Violation Prevention: Zero introduction of unmentioned software libraries, functions, commands, or conceptual frameworks.
  - Textual Reference: "based strictly on the information below — no fabrication, no inference beyond what's shown".

- Constraint: Word Count / Detail Expectation
  - Requirement: Maximum detail density targeting a minimum of 1500 words.
  - Textual Reference: "be as detailed as mostly minimum 1500 words".
  - Conflict Resolution: Because no external project text was supplied, expanding on the given operational requirements, structural definitions, and formatting rules in exhaustive detail fulfills both the factuality directive and detail requirement without fabricating fictional software features.

- Constraint: Terminal Rendering Compatibility
  - Requirement: Render correctly in plain terminal environments without text wrapping degradation.
  - Textual Reference: "FORMATTING RULE: this renders in a plain terminal, not a browser."

- Constraint: Table Elimination
  - Requirement: Absolute ban on standard Markdown tables.
  - Textual Reference: "Do NOT use GFM markdown tables (no pipe-delimited '|' rows) anywhere in the document".
  - Scope of Application: Applies across all sections, specifically including lists of tools, endpoints, or dependencies.
  - Technical Rationale: "wide tables reliably degenerate into broken repeated dash lines in this environment".

- Constraint: Indented List Representation
  - Requirement: Attribute representation via indented bullet points.
  - Textual Reference: "Instead, use a bulleted list with one bullet per item and indented sub-bullets per attribute".

---

### 5.2 Provided Schema Elements

- Element: Primary Bullet Item
  - Format: `- <Item Name>`
  - Function: Serves as the top-level container for a distinct tool, endpoint, module, directive, or configuration parameter.

- Element: Sub-Bullet Attribute - Description
  - Format: `  - Description: <Text>`
  - Function: Contains the explanatory text describing what the parent item is or does.

- Element: Sub-Bullet Attribute - Endpoint
  - Format: `  - Endpoint: <URI/Path>`
  - Function: Denotes the API endpoint, URL path, or location identifier associated with the parent item.

- Element: Sub-Bullet Attribute - Method
  - Format: `  - Method: <HTTP Verb/Execution Action>`
  - Function: Details the HTTP verb (e.g., GET, POST, PUT, DELETE) or computational method associated with the parent item.

---

## 6. Comprehensive Structural Representation

To satisfy the structural format requirement using only the explicitly defined rules, the following list categorizes all prompt parameters into the required bulleted sub-attribute schema:

- Requirement 1: Factuality Strictness
  - Description: All document content must strictly derive from provided text without introducing external assumptions or fabricated facts.
  - Endpoint: N/A (Internal Directive)
  - Method: Constraint Verification

- Requirement 2: Terminal Display Optimization
  - Description: Output format must be compatible with plain terminal text readers and avoid browser-specific table features.
  - Endpoint: N/A (Formatting Directive)
  - Method: Terminal Text Rendering

- Requirement 3: Prohibition of GFM Tables
  - Description: Pipe-delimited markdown table structures must not be used anywhere in the text to avoid display corruption.
  - Endpoint: N/A (Formatting Exclusion)
  - Method: Table Elimination

- Requirement 4: Hierarchical Bullet Layout
  - Description: Information attributes must be formatted as indented sub-bullets beneath a single primary bullet per entry.
  - Endpoint: N/A (Structural Specification)
  - Method: Indented List Structuring

- Requirement 5: Detail Level and Length
  - Description: The document must be as detailed as possible, targeting a minimum length of 1500 words while adhering strictly to non-fabrication.
  - Endpoint: N/A (Quantity Standard)
  - Method: Exhaustive Structural Expansion

---

## 7. Operational Guidelines for Input Data Integration

If additional project documentation, code samples, tool listings, or endpoint specifications are provided in future inputs, they must be formatted using the following bulleted pattern to maintain full compliance with the established terminal rendering rules:

- Generic Tool / Endpoint Template
  - Item Name: `<Name of Tool or Module>`
    - Description: `<Detailed functional summary of the tool or component>`
    - Endpoint: `<Network route, API path, or resource locator>`
    - Method: `<Associated operation type, HTTP method, or function call>`

- Dependency Item Template
  - Item Name: `<Name of Package or Library>`
    - Description: `<Purpose of dependency within the project stack>`
    - Endpoint: `<Repository URL or package registry path, if applicable>`
    - Method: `<Installation or import method>`

- Configuration Parameter Template
  - Item Name: `<Environment Variable or Setting Name>`
    - Description: `<Explanation of what the configuration controls>`
    - Endpoint: `<System path or configuration file location>`
    - Method: `<Set, Read, or Override execution mechanism>`

---

## 8. Explicit Non-Fabrication Summary Audit

- Project Name: Not provided in source text (Omitted to prevent fabrication).
- Installation Steps: Not provided in source text (Omitted to prevent fabrication).
- Configuration Files: Not provided in source text (Omitted to prevent fabrication).
- API Routes / Endpoints: Not provided in source text beyond structural schema example (Omitted to prevent fabrication).
- Dependencies / Prerequisites: Not provided in source text (Omitted to prevent fabrication).
- License Information: Not provided in source text (Omitted to prevent fabrication).
- Usage Instructions: Not provided in source text (Omitted to prevent fabrication).

This README stands complete relative to the source input provided, strictly containing zero fabricated assumptions, zero pipe-delimited markdown tables, and full adherence to plain terminal list formatting.