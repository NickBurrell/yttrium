use servo::compositing::windowing::{WindowMethods, AnimationState, EmbedderCoordinates};
use servo::webrender_surfman::WebrenderSurfman;

struct UnityWindow {}

impl WindowMethods for UnityWindow {
    fn get_coordinates(&self) -> EmbedderCoordinates {
        unimplemented!()
    }

    fn set_animation_state(&self, _state: AnimationState) {
        unimplemented!()
    }

    fn get_gl_context(&self) -> GlContext {
        unimplemented!()
    }

    fn get_native_display(&self) -> NativeDisplay {
        unimplemented!()
    }

    fn get_gl_api(&self) -> GlApi {
        unimplemented!()
    }

    fn webrender_surfman(&self) -> WebrenderSurfman {
        unimplemented!()
    }
}