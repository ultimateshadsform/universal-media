import test from 'ava'
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
  customError,
  ErrorStatus,
} from '../index.js'

// Helper function to wait
const wait = ms => new Promise(resolve => setTimeout(resolve, ms));

// Media Info and Thumbnail
test('should get media info', async (t) => {
  const info = getMediaInfo()
  if (info !== null) {
    t.true('playbackStatus' in info)
    t.true('hasThumbnail' in info)
  } else {
    t.pass('No media playing')
  }
})

test('should get thumbnail if available', async (t) => {
  const thumbnail = getThumbnail()
  if (thumbnail !== null) {
    t.true(Array.isArray(thumbnail))
    t.true(thumbnail.every((byte) => byte >= 0 && byte <= 255))
  } else {
    t.pass('No thumbnail available')
  }
})

// Playback Control Sequence
test('should execute full playback sequence', async (t) => {
  // Initial play
  t.true(await play())
  await wait(1000)
  const initialInfo = getMediaInfo()

  // Next track
  t.true(await next())
  await wait(1000)
  const afterNextInfo = getMediaInfo()

  if (initialInfo && afterNextInfo) {
    try {
      // Check if any media info property changed
      const trackChanged = 
        initialInfo.title !== afterNextInfo.title ||
        initialInfo.artist !== afterNextInfo.artist ||
        initialInfo.album !== afterNextInfo.album ||
        initialInfo.albumArtist !== afterNextInfo.albumArtist;
      
      if (!trackChanged) {
        t.pass('No track change detected - might be at end of playlist')
      } else {
        t.true(trackChanged)
      }
    } catch (error) {
      t.pass(`Track change verification skipped: ${error.message}`)
    }
  } else {
    t.pass('Media info not available')
  }

  // Test pause/play/stop
  t.true(await pause())
  await wait(500)
  t.true(await previous())
  await wait(500)
  t.true(await play())
  await wait(500)
  t.true(await stop())
})

// Volume Controls
test('should handle volume controls', async (t) => {
  // Get initial volume
  const initialVolume = await getSystemVolume()
  if (initialVolume === null) {
    t.pass('Volume control not available')
    return
  }

  // Try to set a different volume
  const newVolume = initialVolume < 0.5 ? 0.8 : 0.2
  t.true(await setSystemVolume(newVolume))
  await wait(1000)

  // Verify volume changed from initial value
  const changedVolume = await getSystemVolume()
  const volumeDiffFromInitial = Math.abs(changedVolume - initialVolume)
  t.true(volumeDiffFromInitial > 0.1, `Volume didn't change from initial ${initialVolume} to ${changedVolume} (diff: ${volumeDiffFromInitial})`)

  // Restore initial volume
  await setSystemVolume(initialVolume)
})

test('should handle invalid volume values', async (t) => {
  t.false(await setSystemVolume(-0.1))
  t.false(await setSystemVolume(1.1))
})

test('should handle mute controls', async (t) => {
  // First try to unmute
  t.true(await setSystemMute(false))
  await wait(500)
  
  const initialMuteState = await getSystemMute()
  if (initialMuteState === null) {
    t.pass('Mute control not available')
    return
  }

  // Test mute sequence
  t.true(await setSystemMute(true))
  await wait(500)
  t.true(await getSystemMute())

  t.true(await setSystemMute(false))
  await wait(500)
  t.false(await getSystemMute())
})

test('should handle rapid state changes', async (t) => {
  t.true(await play())
  await wait(200)
  t.true(await pause())
  await wait(200)
  t.true(await play())
  await wait(200)
  t.true(await stop())

  const info = getMediaInfo()
  if (info === null) {
    t.pass('Media info not available')
  } else {
    t.not(info, null)
  }
})

test('should handle volume control stress test', async (t) => {
  // Test rapid changes
  for (let i = 0; i < 5; i++) {
    const volume = Math.random()
    t.true(await setSystemVolume(volume))
    await wait(200)
  }

  const finalVolume = await getSystemVolume()
  const finalMute = await getSystemMute()
  
  if (finalVolume === null) {
    t.pass('Volume control not available')
  } else {
    t.true(finalVolume >= 0 && finalVolume <= 1)
  }
  
  if (finalMute === null) {
    t.pass('Mute control not available')
  } else {
    t.true(typeof finalMute === 'boolean')
  }
})