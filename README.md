[![Build](https://github.com/javonharper/oxicast/actions/workflows/build.yml/badge.svg)](https://github.com/javonharper/oxicast/actions/workflows/build.yml)

# oxicast

Turn folders of audio files into podcast shows

## What does it do?
- [x] Read a list of show directories from a root directory
- [x] Generate an RSS feed for each show directory `feed.xml`.
- [x] Serve the RSS feed and audio files on a local server at http://127.0.0.1:8080
- [ ] Serve directories for a better user experience
- [ ] Pull metadata tags from audio files to fill metadata

# Usage

Point oxicast to a directory containing your podcast shows:

```bash
oxicast --dir /path/to/your/podcasts
```

### Directory Structure

Organize your audio files like this:

```
podcasts/
├── Show One/
│   ├── episode1.mp3
│   ├── episode2.mp3
│   └── feed.xml (generated)
└── Show Two/
    ├── episode1.mp3
    └── feed.xml (generated)
```

Each subdirectory represents a podcast show. oxicast will generate a `feed.xml` file in each show directory and serve both the feeds and audio files.

### Accessing Your Podcasts

Once running, you can:

1. View the feed at `http://127.0.0.1:8080/Show%20One/feed.xml`
2. Add this URL to your favorite podcast app
3. Access audio files directly at `http://127.0.0.1:8080/Show%20One/episode1.mp3`

> [!NOTE]
> Make sure your podcast app downloads the episodes before shutting down the server.

## License

MIT
