use crate::RESOURCE_BASE_URL;
use js_sys::{ArrayBuffer, Math, Promise, Uint8Array};
use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, AudioBuffer, AudioBufferSourceNode, AudioContext, HtmlAudioElement};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum ClipId {
    CrackleI,
    CrackleII,
    CrackleIII,
    ZapI,
    ZapII,
    ZapIII,
    Beam,
    Diagonal,
    Shield,
    LevelEnter,
    LevelSuccess,
    LevelFailure,
    MageSelect,
    MageDeselect,
    MageMove,
    ClickForward,
    ClickBack,
    ButtonHover,
    MapPlaceObject,
    MapSelectSquare,
    MapIncreaseSize,
    MapDecreaseSize,
    StarSparkle,
}

#[derive(Clone, Debug)]
pub struct AudioClip {
    buffer: AudioBuffer,
    volume: f64,
}

#[derive(Clone, Debug)]
pub struct AudioSystem {
    context: AudioContext,
    audio_clips: HashMap<ClipId, AudioClip>,
}

impl AudioSystem {
    pub async fn register_audio_clip(&mut self, clip_id: ClipId, data: &[u8], volume: f64) {
        let promise = self
            .context
            .decode_audio_data(&u8_slice_to_array_buffer(data))
            .ok();

        if let Some(promise) = promise {
            let buffer = wasm_bindgen_futures::JsFuture::from(promise)
                .await
                .unwrap()
                .dyn_into::<AudioBuffer>()
                .unwrap();

            let audio_clip = AudioClip { buffer, volume };

            console::log_1(&format!("{:?}", audio_clip).into());

            self.audio_clips.insert(clip_id, audio_clip);
        }
    }

    pub fn play_clip(&self, clip_id: ClipId) {
        if let Some(audio_clip) = self.audio_clips.get(&clip_id) {
            // let audio_element = audio_clip.audio_element;
            // audio_clip.audio_element.set_current_time(0.0);
            // let _ = audio_clip.audio_element.play();

            // if let Some(buffer_source) = &audio_clip.buffer_source {
            // } else {
            //     // audio_clip.audio_promise.
            // }

            let buffer_source = self.context.create_buffer_source().unwrap();
            buffer_source.set_buffer(Some(&audio_clip.buffer));

            let _ = buffer_source.start();
        }
    }

    pub fn play_random_zap(&self, hits: usize) {
        let rand = Math::random();

        if rand < 0.33 {
            self.play_clip(ClipId::ZapI);
        } else if rand < 0.66 {
            self.play_clip(ClipId::ZapII);
        } else {
            self.play_clip(ClipId::ZapIII);
        }

        match hits {
            0 => (),
            1 => self.play_clip(ClipId::CrackleI),
            2 => self.play_clip(ClipId::CrackleII),
            _ => self.play_clip(ClipId::CrackleIII),
        }
    }

    pub async fn populate_audio(&mut self) {
        self.register_audio_clip(
            ClipId::CrackleI,
            include_bytes!("../../static/wav/COMBAT_Crackle_1.wav"),
            1.0,
        )
        .await;
        self.register_audio_clip(
            ClipId::CrackleII,
            include_bytes!("../../static/wav/COMBAT_Crackle_2.wav"),
            1.0,
        )
        .await;
        self.register_audio_clip(
            ClipId::CrackleIII,
            include_bytes!("../../static/wav/COMBAT_Crackle_3.wav"),
            1.0,
        )
        .await;
        self.register_audio_clip(
            ClipId::ZapI,
            include_bytes!("../../static/wav/COMBAT_Hit_1.wav"),
            1.0,
        )
        .await;
        self.register_audio_clip(
            ClipId::ZapII,
            include_bytes!("../../static/wav/COMBAT_Hit_2.wav"),
            1.0,
        )
        .await;
        self.register_audio_clip(
            ClipId::ZapII,
            include_bytes!("../../static/wav/COMBAT_Hit_3.wav"),
            1.0,
        )
        .await;
    }
}

impl Default for AudioSystem {
    fn default() -> Self {
        let mut audio_system = Self {
            context: AudioContext::new().unwrap(),
            audio_clips: Default::default(),
        };

        audio_system
    }
}

fn u8_slice_to_array_buffer(u8_slice: &[u8]) -> ArrayBuffer {
    let uint8_array = Uint8Array::new_with_length(u8_slice.len() as u32);
    uint8_array.set(&Uint8Array::from(u8_slice), 0);
    ArrayBuffer::from(uint8_array.buffer())
}
