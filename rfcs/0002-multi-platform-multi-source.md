# Multiple Platform and Multiple Source

- Feature Name: Multiple Platform and Multiple Source
- Start Date: 2023-02-24

## Summary

We can support multiple platform: web, tauri, electron, etc. Different platform may need different data persistence. For example, we can use sqlite in tauri, but we can use indexDB in web.

We can add multiple source into the Musync client. For example, netease remote server, jellyfin, local directory, etc.
If the source support multiple account, we can treat different user in a source as different source.

This rfc also deprecates the rfc-0001.

## Motivation

## Guide-level Explanation

Different platform should has different behavior.

### Available source in platforms

- Web
  - netease http api
  - jellyfin
- Local(tauri)
  - netease http api
  - netease local api
  - jellyfin
  - local directory

### Track ID


Track should has unified ID to identify the same track in different track.
