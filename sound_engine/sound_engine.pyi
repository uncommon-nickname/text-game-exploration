class AudioPlayer:
    def queue_sound(self, path: str, *, repeat: bool = False) -> None:
        """Adds a sound to the internal play queue.

        Arguments:
            path: A relative/absolute path to the sound asset.
            repeat: If set, plays a sound in an infinite loop.
        """
    def set_volume(self, volume: float = 1.0) -> None:
        """Changes the playback volume.

        Arguments:
            volume: A float value representing the percentage of the
                playback volume (1.0 -> 100%).
        """
    def play(self) -> None:
        """Resumes to play the currently processed sound.

        Does nothing if any sound is already playing.
        """
    def pause(self) -> None:
        """Pauses the currently playing sound and remembers its state.

        Please be informed that paused sound is kept loaded into the
        memory and this should be used only when a short pause is
        needed.
        """
    def stop(self) -> None:
        """Stops to play and clears the execution queue.

        Immidiately halts the device and clears any queued sounds that
        still linger in the queue. The AudioPlayer object is still valid
        and can be reused.
        """

class AudioEngine:
    def new_concurrent_player(self) -> AudioPlayer:
        """Creates a new audio player object.

        Every new audio player uses the same (default) audio device but
        has its own execution queue and runs on a separate thread.
        """
