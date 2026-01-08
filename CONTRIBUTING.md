# Contributing

We currently have the following viewers implemented:

- **JSON**: [lat-json-viewer](https://github.com/bglgwyng/lat-json-viewer)
- **JS/TS**: [lat-js-viewer](https://github.com/bglgwyng/lat-js-viewer)
- **Plaintext (fallback)**: [lat-plaintext-viewer](https://github.com/bglgwyng/lat-plaintext-viewer)

## Adding Support for New File Types

To extend `lat` to support more file extensions, you can create your own viewer. We welcome pull requests for both new viewers and improvements to existing ones!

## Distribution Notes

For the Nix distribution, users will be able to add custom file extension support and replace existing viewers with their own implementations. Non-Nix distributions will ship with a fixed set of standard viewers and won't be configurable.
