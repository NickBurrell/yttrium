#![feature(box_syntax)]
#![allow(dead_code)]

mod unity;
mod runtime;
mod dotnet;

impl std::convert::Into<i32> for unity::ffi::UnityRenderingExtEventType {
    fn into(self) -> i32 {
        use unity::ffi::UnityRenderingExtEventType;
        match self {
            UnityRenderingExtEventType::kUnityRenderingExtEventSetStereoTarget => 0,
            UnityRenderingExtEventType::kUnityRenderingExtEventSetStereoEye => 1,
            UnityRenderingExtEventType::kUnityRenderingExtEventStereoRenderingDone => 2,
            UnityRenderingExtEventType::kUnityRenderingExtEventBeforeDrawCall => 3,
            UnityRenderingExtEventType::kUnityRenderingExtEventAfterDrawCall => 4,
            UnityRenderingExtEventType::kUnityRenderingExtEventCustomGrab => 5,
            UnityRenderingExtEventType::kUnityRenderingExtEventCustomBlit => 6,
            UnityRenderingExtEventType::kUnityRenderingExtEventUpdateTextureBegin => 7,
            UnityRenderingExtEventType::kUnityRenderingExtEventUpdateTextureEnd => 8,
            UnityRenderingExtEventType::kUnityRenderingExtEventUpdateTextureBeginV2 => 9,
            UnityRenderingExtEventType::kUnityRenderingExtEventUpdateTextureEndV2 => 10,
            UnityRenderingExtEventType::kUnityRenderingExtEventCount => 11,
            _ => -1,
        }
    }
}