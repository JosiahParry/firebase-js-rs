use crate::app::FirebaseApp;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Auth;

    #[wasm_bindgen(method, js_name = auth)]
    pub fn auth(_: &FirebaseApp) -> Auth;

    #[wasm_bindgen(method, js_name = onAuthStateChanged)]
    pub fn on_auth_state_changed(this: &Auth, callback: &Closure<dyn FnMut(JsValue)>);

    #[wasm_bindgen(method, catch, js_name = createUserWithEmailAndPassword)]
    pub async fn create_user_with_email_and_password(
        this: &Auth,
        email: String,
        password: String,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = signInWithEmailAndPassword)]
    pub async fn sign_in_with_email_and_password(
        this: &Auth,
        email: String,
        password: String,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = signOut)]
    pub async fn sign_out(this: &Auth) -> Result<JsValue, JsValue>;

    // Google Auth support
    pub type GoogleAuthProvider;

    #[wasm_bindgen(constructor)]
    pub fn new() -> GoogleAuthProvider;

    // Github Auth support
    pub type GithubAuthProvider;

    #[wasm_bindgen(constructor)]
    pub fn new() -> GithubAuthProvider;

    #[wasm_bindgen(catch, method, js_name = signInWithPopup)]
    pub async fn sign_in_with_popup(
        auth: &Auth,
        provider: GoogleAuthProvider,
    ) -> Result<JsValue, JsValue>;
}
