# Spec Delta

## ADDED Requirements

### Requirement: Help pages and examples in the package
Each released `.vsix` SHALL hold the help pages made from the language reference, their index, the reference's
example programs with their recorded output, and the example programs of `examples\`. It SHALL hold no text of
the QB64pe wiki. The pages SHALL be made from the reference at the commit the package is built from.

#### Scenario: Listing the package
- **WHEN** the files of the released `.vsix` are listed
- **THEN** they include the help index, the page of `_CLAMP`, the example `_clamp_1.bas` and the "Hello"
  example

#### Scenario: A page changed in the repository
- **WHEN** the reference page of `_CLAMP` is edited and the package is built again
- **THEN** the package's page of `_CLAMP` holds the edit
