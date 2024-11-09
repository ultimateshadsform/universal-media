# universal-media 🎵

A Node.js native module for controlling media playback and system audio. Built with Rust and OS specific APIs.

## Features 🚀

### Media Control 🎮

- Get current media info (title, artist, album) 📝
- Get media thumbnail 🖼️
- Play/Pause/Stop control ⏯️
- Next/Previous track navigation ⏭️
- Get playback status 📊

### Volume Control 🔊

- Get/Set system volume 🎚️
- Get/Set system mute status 🔇
- [TODO] Control individual application volumes 🎛️

## Installation 📦

> [!NOTE]
> This is a work in progress. API might change rapidly between releases.
> Right now only Windows is supported.

```bash
npm install @ultimateshadsform/universal-media
```

## Usage 📖

```typescript
import { getMediaInfo, getThumbnail } from '@ultimateshadsform/universal-media';
import fs from 'fs';

const mediaInfo = getMediaInfo();
const thumbnail = getThumbnail();

console.log(mediaInfo); // { title: 'Song Title', artist: 'Artist Name', album: 'Album Name', playbackStatus: 'playing', hasThumbnail: true }

const thumbnailBuffer = Buffer.from(thumbnail);
fs.writeFileSync('thumbnail.png', thumbnailBuffer);
```

## Contributing 🤝

See [CONTRIBUTING.md](CONTRIBUTING.md)

## License 📄

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
