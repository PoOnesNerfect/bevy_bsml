# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

[unreleased]: https://github.com/PoOnesNerfect/bevy_bsml

## [0.14.1] - 2024-08-06

### Changed

- Add more classes + Commit Cargo.lock ([#5])
- Reorganized bsml macro for better readability
- Renamed all instances of `Color::rgba` to `Color::Srgba`.
- Renamed `ClassEnum` into a more fitting name, `BsmlClass`.
- `fn apply_class_system`, which applies any interactions classes changes that happend in Bsml elements,
  will not run in `PostUpdate` instead of `Update`.
- Add more tailwind classes:
  - MaxWidth
  - AlignItems
  - Position (Absolute, Relative)
  - Bottom, Top, Left, Right
  - JustifyText
- **BREAKING**: Renamed struct `BsmlNode` to just `Bsml`.
- **BREAKING**: Renamed trait `Bsml` to `BsmlElement`.
- **BREAKING**: Renamed module `class::text::color` to `class::text::text_color`.
- **BREAKING**: Refactored class applying system.
  - Instead of querying different components for `background color`, `styles`, `text`,
    etc., you can just query `&mut BsmlClasses` and insert/remove the classes you want.
  - See [examples directory] for more information.

[#5]: https://github.com/PoOnesNerfect/bevy_bsml/pull/5
[examples directory]: https://github.com/PoOnesNerfect/bevy_bsml/tree/main/examples
[0.14.1]: https://github.com/PoOnesNerfect/bevy_bsml/compare/v0.14.0...v0.14.1

## [0.14.0] - 2024-07-17

Starting this version, this project will keep the same minor version as Bevy.
This allows me to pushing new features while still supporting each versions of Bevy.

[0.14.0]: https://github.com/PoOnesNerfect/bevy_bsml/compare/v0.0.8...v0.14.0

### Added

- Starting a changelog
  - I love reading through other projects' changelogs for added features and bugfixes, so I hope
    this will be useful to others as well.

### Changed

- Updated Bevy version to 0.14 ([#3])

[#3]: https://github.com/PoOnesNerfect/bevy_bsml/pull/3

## [0.0.8]

### Changed

- Updated Bevy version to 0.13 ([#1])

[#1]: https://github.com/PoOnesNerfect/bevy_bsml/pull/1
[0.0.8]: https://github.com/PoOnesNerfect/bevy_bsml/compare/v0.0.7...v0.0.8
