//! Zero-Copy Circular Pre-Roll Buffer and Event Clip Recorder.
//!
//! Maintains an in-memory ring buffer of recent H.264 NALUs to capture pre-roll footage
//! upon trigger (motion, meteor, or REST request) without persistent flash wear.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct BufferedNalu {
    pub data: Vec<u8>,
    pub timestamp: Instant,
    pub is_keyframe: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipMetadata {
    pub filename: String,
    pub path: String,
    pub size_bytes: u64,
    pub duration_secs: f64,
    pub trigger_label: String,
}

pub struct EventClipRecorder {
    pre_roll_duration: Duration,
    ring_buffer: VecDeque<BufferedNalu>,
    recording_dir: PathBuf,
    active_recording: Option<ActiveRecording>,
}

struct ActiveRecording {
    label: String,
    start_time: Instant,
    deadline: Instant,
    collected_nalus: Vec<Vec<u8>>,
}

impl EventClipRecorder {
    pub fn new<P: AsRef<Path>>(recording_dir: P, pre_roll_secs: f64) -> Self {
        let dir = recording_dir.as_ref().to_path_buf();
        let _ = fs::create_dir_all(&dir);
        Self {
            pre_roll_duration: Duration::from_secs_f64(pre_roll_secs),
            ring_buffer: VecDeque::new(),
            recording_dir: dir,
            active_recording: None,
        }
    }

    /// Pushes an incoming H.264 NALU into the circular pre-roll buffer.
    pub fn push_nalu(&mut self, nalu: Vec<u8>) {
        let now = Instant::now();
        let is_keyframe = is_h264_keyframe(&nalu);

        // If an active recording is in progress, accumulate post-roll NALU
        if let Some(rec) = &mut self.active_recording {
            rec.collected_nalus.push(nalu.clone());
        }

        self.ring_buffer.push_back(BufferedNalu {
            data: nalu,
            timestamp: now,
            is_keyframe,
        });

        // Prune older than pre_roll_duration
        while let Some(front) = self.ring_buffer.front() {
            if front.timestamp.elapsed() > self.pre_roll_duration {
                self.ring_buffer.pop_front();
            } else {
                break;
            }
        }
    }

    /// Triggers an event recording with specified label and post-roll duration.
    pub fn trigger_event(&mut self, label: &str, post_roll_secs: f64) -> Result<(), String> {
        let now = Instant::now();
        let deadline = now + Duration::from_secs_f64(post_roll_secs);

        // Extract pre-roll NALUs starting from the earliest keyframe in buffer
        let mut pre_roll = Vec::new();
        let mut keyframe_found = false;

        for item in &self.ring_buffer {
            if !keyframe_found && item.is_keyframe {
                keyframe_found = true;
            }
            if keyframe_found {
                pre_roll.push(item.data.clone());
            }
        }

        // If no keyframe was in the buffer, include all available
        if !keyframe_found {
            for item in &self.ring_buffer {
                pre_roll.push(item.data.clone());
            }
        }

        self.active_recording = Some(ActiveRecording {
            label: label.to_string(),
            start_time: now,
            deadline,
            collected_nalus: pre_roll,
        });

        Ok(())
    }

    /// Polls the recorder. If active recording deadline passed, writes clip to disk.
    pub fn poll_finalize(&mut self) -> Option<ClipMetadata> {
        let is_finished = if let Some(rec) = &self.active_recording {
            Instant::now() >= rec.deadline
        } else {
            false
        };

        if is_finished {
            let rec = self.active_recording.take().unwrap();
            let timestamp_str = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
            let filename = format!("clip_{}_{}.h264", rec.label, timestamp_str);
            let filepath = self.recording_dir.join(&filename);

            let mut total_bytes = 0u64;
            let mut file_data = Vec::new();
            for nalu in rec.collected_nalus {
                file_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]); // Annex-B start code
                file_data.extend_from_slice(&nalu);
                total_bytes += 4 + nalu.len() as u64;
            }

            if fs::write(&filepath, &file_data).is_ok() {
                return Some(ClipMetadata {
                    filename,
                    path: filepath.to_string_lossy().to_string(),
                    size_bytes: total_bytes,
                    duration_secs: rec.start_time.elapsed().as_secs_f64(),
                    trigger_label: rec.label,
                });
            }
        }

        None
    }

    /// Lists all recorded clips in the recording directory.
    pub fn list_clips(&self) -> Vec<ClipMetadata> {
        let mut clips = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.recording_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "h264") {
                    let filename = path.file_name().unwrap().to_string_lossy().to_string();
                    let metadata = entry.metadata().ok();
                    let size_bytes = metadata.as_ref().map_or(0, |m| m.len());
                    clips.push(ClipMetadata {
                        filename,
                        path: path.to_string_lossy().to_string(),
                        size_bytes,
                        duration_secs: 5.0, // Approximate
                        trigger_label: "event".to_string(),
                    });
                }
            }
        }
        clips
    }
}

/// Checks if an H.264 NALU is an IDR picture (NAL type 5) or SPS (type 7).
pub fn is_h264_keyframe(nalu: &[u8]) -> bool {
    if nalu.is_empty() {
        return false;
    }
    let nal_type = nalu[0] & 0x1F;
    nal_type == 5 || nal_type == 7
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_pre_roll_ring_buffer_and_trigger() {
        let dir = tempdir().unwrap();
        let mut recorder = EventClipRecorder::new(dir.path(), 2.0);

        // Push keyframe (SPS type 7) and slice (type 1)
        recorder.push_nalu(vec![0x67, 0x42, 0x00]); // SPS
        recorder.push_nalu(vec![0x65, 0x88, 0x84]); // IDR
        recorder.push_nalu(vec![0x41, 0x9A]);       // Non-IDR

        assert_eq!(recorder.ring_buffer.len(), 3);

        // Trigger recording with 0s post-roll for immediate test
        recorder.trigger_event("motion", 0.0).unwrap();
        let clip = recorder.poll_finalize();

        assert!(clip.is_some());
        let meta = clip.unwrap();
        assert!(meta.filename.starts_with("clip_motion"));
        assert!(meta.size_bytes > 0);
    }
}
