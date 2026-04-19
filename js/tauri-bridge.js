import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export function initTauriBridge(player) {
    const getTrackTitle = (track) => {
        if (!track) return "";
        return track.title || "Unknown Title";
    };

    const getTrackArtists = (track) => {
        if (!track) return "";
        if (track.artists && track.artists.length > 0) {
            return track.artists.map(a => a.name).join(", ");
        }
        if (track.artist && track.artist.name) {
            return track.artist.name;
        }
        return "Unknown Artist";
    };

    // === OUTBOUND: Player → Tray ===
    const sendState = () => {
        if (!player || !player.audio) return;
        
        invoke('set_playback_state', {
            state: {
                isPlaying: !player.audio.paused,
                title: getTrackTitle(player.currentTrack),
                artist: getTrackArtists(player.currentTrack),
                volume: player.userVolume,
                isMuted: player.audio.volume === 0 || player.userVolume === 0,
            }
        }).catch(err => console.error("Failed to send playback state to Tauri:", err));
    };
    
    player.audio.addEventListener('play', sendState);
    player.audio.addEventListener('pause', sendState);
    player.audio.addEventListener('volumechange', sendState);
    player.audio.addEventListener('loadedmetadata', sendState);

    // Also send initial state
    sendState();

    // === INBOUND: Tray → Player ===
    listen('tray-play-pause', () => {
        player.handlePlayPause();
    });
    
    listen('tray-next', () => {
        player.playNext();
    });
    
    listen('tray-prev', () => {
        player.playPrev();
    });
    
    listen('tray-mute', () => {
        if (player.audio.volume === 0 || player.userVolume === 0) {
            // Unmute - assuming player.userVolume was saved before mute or it has a toggle
            // Monochrome's player has userVolume and actual audio.volume
            player.setVolume(player.userVolume > 0 ? player.userVolume : 1.0);
        } else {
            // Mute
            player.audio.volume = 0;
            // Ensure UI updates if needed
        }
    });
    
    listen('tray-volume-up', () => {
        const newVol = Math.min(1.0, player.userVolume + 0.1);
        player.setVolume(newVol);
    });
    
    listen('tray-volume-down', () => {
        const newVol = Math.max(0.0, player.userVolume - 0.1);
        player.setVolume(newVol);
    });

    // === SETTINGS UI: Close to Tray ===
    setTimeout(() => {
        const instancesTab = document.getElementById('settings-tab-instances');
        if (instancesTab) {
            const group = instancesTab.querySelector('.settings-group') || instancesTab;
            const settingHtml = `
            <div class="setting-item" id="tauri-close-to-tray-setting">
                <div class="info">
                    <span class="label">Close to Tray</span>
                    <span class="description">Minimize to system tray instead of quitting when closing the window.</span>
                </div>
                <label class="toggle-switch">
                    <input type="checkbox" id="tauri-close-to-tray-toggle" />
                    <span class="slider"></span>
                </label>
            </div>`;
            group.insertAdjacentHTML('afterbegin', settingHtml);
            
            const toggle = document.getElementById('tauri-close-to-tray-toggle');
            if (toggle) {
                invoke('get_close_to_tray_setting').then(value => {
                    toggle.checked = value;
                });
                toggle.addEventListener('change', (e) => {
                    invoke('set_close_to_tray_setting', { enabled: e.target.checked }).catch(console.error);
                });
            }
        }
    }, 1000);
}
