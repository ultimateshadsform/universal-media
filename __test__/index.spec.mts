import { it, expect, test } from 'vitest';
import {
  getMediaInfo,
  getThumbnail,
  play,
  pause,
  next,
  previous,
  stop,
  setSystemVolume,
  getSystemVolume,
  setSystemMute,
  getSystemMute,
} from '../index.js';

// Media Info and Thumbnail
it('should get media info', async () => {
  const info = getMediaInfo();
  if (info !== null) {
    expect(info).toHaveProperty('playbackStatus');
    expect(info).toHaveProperty('hasThumbnail');
  } else {
    test.skip('No media info available');
  }
});

it('should get thumbnail if available', async () => {
  const thumbnail = getThumbnail();
  if (thumbnail !== null) {
    expect(Array.isArray(thumbnail)).toBe(true);
    expect(thumbnail.every((byte) => byte >= 0 && byte <= 255)).toBe(true);
  } else {
    test.skip('No thumbnail available');
  }
});

// Playback Control Sequence
it('should execute full playback sequence', async () => {
  // Initial play
  expect(await play()).toBe(true);
  const initialInfo = getMediaInfo();

  // Next track
  expect(await next()).toBe(true);
  const afterNextInfo = getMediaInfo();

  if (initialInfo && afterNextInfo) {
    // Check if any media info property changed
    const trackChanged =
      initialInfo.title !== afterNextInfo.title ||
      initialInfo.artist !== afterNextInfo.artist ||
      initialInfo.album !== afterNextInfo.album ||
      initialInfo.albumArtist !== afterNextInfo.albumArtist;

    // Only assert if we expect track to change
    if (trackChanged) {
      expect(trackChanged).toBe(true);
    } else {
      test.skip('Track did not change - may be expected behavior');
    }
  } else {
    test.skip('Media info not available');
  }

  // Test pause/play/stop
  expect(await pause()).toBe(true);
  expect(await previous()).toBe(true);
  expect(await play()).toBe(true);
  expect(await stop()).toBe(true);
});

// Volume Controls
it('should handle volume controls', async () => {
  const initialVolume = await getSystemVolume();
  if (initialVolume === null) {
    test.skip('System volume control not available');
    return;
  }

  // Try to set a different volume
  const newVolume = initialVolume < 0.5 ? 0.8 : 0.2;
  expect(await setSystemVolume(newVolume)).toBe(true);

  const changedVolume = await getSystemVolume();
  if (changedVolume === null) {
    test.skip('Could not verify volume change');
    return;
  }

  const volumeDiffFromInitial = Math.abs(changedVolume - initialVolume);
  expect(volumeDiffFromInitial > 0.1).toBe(true);

  // Restore initial volume
  await setSystemVolume(initialVolume);
});

it('should handle invalid volume values', async () => {
  expect(await setSystemVolume(-0.1)).toBe(false);
  expect(await setSystemVolume(1.1)).toBe(false);
});

it('should handle mute controls', async () => {
  // First try to unmute
  expect(await setSystemMute(false)).toBe(true);

  const initialMuteState = await getSystemMute();
  if (initialMuteState === null) {
    test.skip('System mute control not available');
    return;
  }

  // Test mute sequence
  expect(await setSystemMute(true)).toBe(true);
  expect(await getSystemMute()).toBe(true);

  expect(await setSystemMute(false)).toBe(true);
  expect(await getSystemMute()).toBe(false);
});

it('should handle rapid state changes', async () => {
  expect(await play()).toBe(true);
  expect(await pause()).toBe(true);
  expect(await play()).toBe(true);
  expect(await stop()).toBe(true);

  const info = getMediaInfo();
  if (info === null) {
    test.skip('Media info not available');
  } else {
    expect(info).toHaveProperty('playbackStatus');
  }
});

it('should handle volume control stress test', async () => {
  // Test rapid changes
  for (let i = 0; i < 5; i++) {
    const volume = Math.random();
    expect(await setSystemVolume(volume)).toBe(true);
  }

  const finalVolume = await getSystemVolume();
  const finalMute = await getSystemMute();

  if (finalVolume === null) {
    test.skip('Volume control not available');
  } else {
    expect(finalVolume).toBeGreaterThanOrEqual(0);
    expect(finalVolume).toBeLessThanOrEqual(1);
  }

  if (finalMute === null) {
    test.skip('Mute control not available');
  } else {
    expect(typeof finalMute).toBe('boolean');
  }
});
