# Change Log
## [1.0.1] - 2023-04-16
- Fix json parsing error:
  Use appropriate `u32`-integer for field `time_until_update`, instead of `u16` which was sometimes too small.
- Various CI improvements
## [1.0.0] - 2023-04-10
- Make modules publicly accessible
- Derive Eq and PartialEq for wrapper, but stop deriving Serialize
- Deserialize wrapper into appropriate data types instead of String
- Provide sample code in README.md
## [0.1.0] - 2023-02-06
Initial Release
