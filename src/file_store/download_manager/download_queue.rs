use std::collections::VecDeque;

use crate::media_sources::soundgasm::SoundgasmAudioTrack;

struct DownloadQueue {
	queue: VecDeque<SoundgasmAudioTrack>,
}

impl DownloadQueue {
	pub fn new() -> Self {
		Self {
			queue: VecDeque::with_capacity(128),
		}
	}

	pub fn enqueue(&mut self, item: SoundgasmAudioTrack) {
		self.queue.push_back(item);
	}

	pub fn dequeue(&mut self) -> Option<SoundgasmAudioTrack> {
		self.queue.pop_front()
	}

	pub fn is_empty(&self) -> bool {
		self.queue.is_empty()
	}

	pub fn len(&self) -> usize {
		self.queue.len()
	}
}
