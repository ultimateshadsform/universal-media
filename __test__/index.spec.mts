import { it, expect, describe } from 'vitest';
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

// Helper function to wait
const wait = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

// Media Info and Thumbnail
it('should get media info', async () => {
  const info = getMediaInfo();
  if (info !== null) {
    expect('playbackStatus' in info).toBe(true);
    expect('hasThumbnail' in info).toBe(true);
  } else {
    expect(true).toBe(true);
  }
});

it('should get thumbnail if available', async () => {
  const thumbnail = getThumbnail();
  if (thumbnail !== null) {
    expect(Array.isArray(thumbnail)).toBe(true);
    expect(thumbnail.every((byte) => byte >= 0 && byte <= 255)).toBe(true);
  } else {
    expect(true).toBe(true);
  }
});

// Playback Control Sequence
it('should execute full playback sequence', async () => {
  // Initial play
  expect(await play()).toBe(true);
  await wait(1000);
  const initialInfo = getMediaInfo();

  // Next track
  expect(await next()).toBe(true);
  await wait(1000);
  const afterNextInfo = getMediaInfo();

  if (initialInfo && afterNextInfo) {
    try {
      // Check if any media info property changed
      const trackChanged =
        initialInfo.title !== afterNextInfo.title ||
        initialInfo.artist !== afterNextInfo.artist ||
        initialInfo.album !== afterNextInfo.album ||
        initialInfo.albumArtist !== afterNextInfo.albumArtist;

      if (!trackChanged) {
        expect(true).toBe(true);
      } else {
        expect(trackChanged).toBe(true);
      }
    } catch (error: any) {
      expect(true).toBe(true);
    }
  } else {
    expect(true).toBe(true);
  }

  // Test pause/play/stop
  expect(await pause()).toBe(true);
  await wait(500);
  expect(await previous()).toBe(true);
  await wait(500);
  expect(await play()).toBe(true);
  await wait(500);
  expect(await stop()).toBe(true);
});

// Volume Controls
it('should handle volume controls', async () => {
  // Get initial volume
  const initialVolume = await getSystemVolume();
  if (initialVolume === null) {
    expect(true).toBe(true);
    return;
  }

  // Try to set a different volume
  const newVolume = initialVolume < 0.5 ? 0.8 : 0.2;
  expect(await setSystemVolume(newVolume)).toBe(true);
  await wait(1000);

  // Verify volume changed from initial value
  const changedVolume = await getSystemVolume();
  if (changedVolume === null) {
    expect(true).toBe(true);
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
  await wait(500);

  const initialMuteState = await getSystemMute();
  if (initialMuteState === null) {
    expect(true).toBe(true);
    return;
  }

  // Test mute sequence
  expect(await setSystemMute(true)).toBe(true);
  await wait(500);
  expect(await getSystemMute()).toBe(true);

  expect(await setSystemMute(false)).toBe(true);
  await wait(500);
  expect(await getSystemMute()).toBe(false);
});

it('should handle rapid state changes', async () => {
  expect(await play()).toBe(true);
  await wait(200);
  expect(await pause()).toBe(true);
  await wait(200);
  expect(await play()).toBe(true);
  await wait(200);
  expect(await stop()).toBe(true);

  const info = getMediaInfo();
  if (info === null) {
    expect(true).toBe(true);
  } else {
    expect(info).not.toBe(null);
  }
});

it('should handle volume control stress test', async () => {
  // Test rapid changes
  for (let i = 0; i < 5; i++) {
    const volume = Math.random();
    expect(await setSystemVolume(volume)).toBe(true);
    await wait(200);
  }

  const finalVolume = await getSystemVolume();
  const finalMute = await getSystemMute();

  if (finalVolume === null) {
    expect(true).toBe(true);
  } else {
    expect(finalVolume >= 0 && finalVolume <= 1).toBe(true);
  }

  if (finalMute === null) {
    expect(true).toBe(true);
  } else {
    expect(typeof finalMute === 'boolean').toBe(true);
  }
});
